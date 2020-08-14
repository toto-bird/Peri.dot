##########################################
# DEPENDENCIES                           #
##########################################

from .constants  import * # type: ignore
from .exceptions import * # type: ignore
from .tokens     import * # type: ignore
from .types      import * # type: ignore

##########################################
# RUNTIME RESULT                         #
##########################################

class RTResult():
    def __init__(self):
        self.value = None
        self.error = None

    def register(self, res):
        if res.error:
            self.error = res.error

        return(res.value)

    def success(self, value):
        self.value = value

        return(self)

    def failure(self, error):
        self.error = error

        return(self)

##########################################
# INTERPRETER                            #
##########################################

class Interpreter():
    def visit(self, node, context):
        method = f'visit_{type(node).__name__}'
        method = getattr(self, method)

        return(
            method(node, context)
        )



    def visit_IntNode(self, node, context):
        return(
            RTResult().success(
                IntType(node.token.value)
                    .setcontext(context)
                    .setpos(node.start, node.end)
            )
        )


    def visit_FloatNode(self, node, context):
        return(
            RTResult().success(
                FloatType(node.token.value)
                    .setcontext(context)
                    .setpos(node.start, node.end)
            )
        )


    def visit_StringNode(self, node, context):
        return(
            RTResult().success(
                StringType(node.token.value)
                    .setcontext(context)
                    .setpos(node.start, node.end)
            )
        )



    def visit_VarAccessNode(self, node, context):
        res = RTResult()

        name = node.token.value

        value = BUILTINS.get(name, None)
        if value:
            value.setpos(node.token.start, node.token.end)
            value.setcontext(context)
            return(
                res.success(
                    value
                )
            )

        value = context.symbols.access(name)

        if not value:
            return(
                res.failure(
                    Exc_IdentifierError(
                        f'\'{name}\' is not defined',
                        node.token.start, node.token.end
                    )
                )
            )

        return(
            res.success(
                value
            )
        )


    def visit_VarAssignNode(self, node, context):
        res = RTResult()

        for i in node.tokens:
            name = i.value

            value = res.register(
                self.visit(
                    node.valnode,
                    context
                )
            )

            if name in list(BUILTINS.keys()) or name in list(BUILTINFUNCS.keys()):
                return(
                    res.failure(
                        Exc_TypeError(
                            f'Can not assign {value.type} to \'{name}\' (reserved)',
                            i.start, i.end
                        )
                    )
                )

            prevvalue = context.symbols.access(name)

            if not prevvalue:
                return(
                    res.failure(
                        Exc_IdentifierError(
                            f'\'{name}\' is not defined',
                            i.start, i.end
                        )
                    )
                )

            if type(prevvalue) != type(value):
                return(
                    res.failure(
                        Exc_TypeError(
                            f'Can not assign {value.type} to \'{name}\' ({prevvalue.type})',
                            node.valnode.token.start, node.valnode.token.end
                        )
                    )
                )

            if res.error:
                return(res)

            context.symbols.assign(name, value)

        return(
            res.success(
                value
            )
        )


    def visit_VarCreateNode(self, node, context):
        res = RTResult()

        for i in node.tokens:
            name = i.value
            value = res.register(
                self.visit(
                    node.valnode,
                    context
                )
            )

            if name in list(BUILTINS.keys()) or name in list(BUILTINFUNCS.keys()):
                return(
                    res.failure(
                        Exc_TypeError(
                            f'Can not assign {value.type} to \'{name}\' (reserved)',
                            i.start, i.end
                        )
                    )
                )

            if res.error:
                return(res)

            context.symbols.assign(name, value)

        return(
            res.success(
                value
            )
        )


    def visit_VarNullNode(self, node, context):
        res = RTResult()

        for i in node.tokens:
            name = i.value

            if name in list(BUILTINS.keys()) or name in list(BUILTINFUNCS.keys()):
                return(
                    res.failure(
                        Exc_TypeError(
                            f'Can not assign {TYPES["nonetype"]} to \'{name}\' (reserved)',
                            i.start, i.end
                        )
                    )
                )

            context.symbols.assign(name, NullType())

        return(
            res.success(
                NullType()
            )
        )


    def visit_VarCallNode(self, node, context):
        res = RTResult()

        name = node.name
        args = node.argnodes
        options = node.optionnodes

        if name == BUILTINFUNCS['assert']:
            if len(args) != 1:
                return(
                    res.failure(
                        Exc_ArgumentError(
                            f'\'{name}\' takes 1 arguments, {len(args)} given',
                            node.start, node.end
                        )
                    )
                )

            defoptions = {'msg': StringType('')}
            for i in range(len(list(options.keys()))):
                op = list(options.keys())[i]
                options[op] = res.register(
                    self.visit(
                        options[op], context
                    )
                )

                if res.error:
                    return(res)

                default = defoptions.get(
                    op,
                    None
                )

                if default:
                    if type(options[op]) == type(default):
                        defoptions[op] = options[op]
                    else:
                        return(
                            res.failure(
                                Exc_TypeError(
                                    f'\'{op}\' is a {default.type} option',
                                    node.start, node.end
                                )
                            )
                        )
                else:
                    return(
                        res.failure(
                            Exc_ArgumentError(
                                f'Invalid option \'{op}\' given',
                                node.start, node.end
                            )
                        )
                    )

            for i in range(len(args)):
                arg = args[i]
                result = res.register(
                    self.visit(
                        arg, context
                    )
                )

                if res.error:
                    return(res)

                args[i] = result

            result, error = args[0].istrue()

            if error:
                return(
                    res.failure(
                        error
                    )
                )

            if result:
                return(
                    res.success(
                        NullType()
                    )
                )

            else:
                return(
                    res.failure(
                        Exc_AssertionError(
                            defoptions['msg'].value,
                            args[0].start, args[0].end
                        )
                    )
                )

        else:
            result = res.register(
                self.visit(
                    node.token, context
                )
            )

            if res.error:
                return(res)

            result, error = result.call()

            if error:
                return(
                    res.failure(
                        error
                    )
                )



    def visit_UnaryOpNode(self, node, context):
        res = RTResult()
        result = res.register(
            self.visit(
                node.node,
                context
            )
        )

        if res.error:
            return(res)

        error = None

        if node.optoken.type == TT_DASH:
            result, error = result.multiply(IntType(-1))
        elif node.optoken.matches(TT_KEYWORD, KEYWORDS['logicalnot']):
            result, error = result.not_()

        if error:
            return(
                res.failure(
                    error
                )
            )

        return(
            res.success(
                result.setpos(
                    node.start,
                    node.end
                )
            )
        )


    def visit_BinaryOpNode(self, node, context):
        res = RTResult()

        left = res.register(
            self.visit(
                node.lnode,
                context
            )
        )
        if res.error:
            return(res)

        right = res.register(
            self.visit(
                node.rnode,
                context
            )
        )
        if res.error:
            return(res)

        if node.optoken.type == TT_PLUS:
            result, error = left.add(right)
        elif node.optoken.type == TT_DASH:
            result, error = left.subtract(right)
        elif node.optoken.type == TT_ASTRISK:
            result, error = left.multiply(right)
        elif node.optoken.type == TT_FSLASH:
            result, error = left.divide(right)
        elif node.optoken.type == TT_CARAT:
            result, error = left.raised(right)
        elif node.optoken.type == TT_EQEQUALS:
            result, error = left.eqequals(right)
        elif node.optoken.type == TT_BANGEQUALS:
            result, error = left.bangequals(right)
        elif node.optoken.type == TT_LESSTHAN:
            result, error = left.lessthan(right)
        elif node.optoken.type == TT_GREATERTHAN:
            result, error = left.greaterthan(right)
        elif node.optoken.type == TT_LTEQUALS:
            result, error = left.ltequals(right)
        elif node.optoken.type == TT_GTEQUALS:
            result, error = left.gtequals(right)
        elif node.optoken.matches(TT_KEYWORD, KEYWORDS['logicaland']):
            result, error = left.and_(right)
        elif node.optoken.matches(TT_KEYWORD, KEYWORDS['logicalor']):
            result, error = left.or_(right)

        if error:
            return(
                res.failure(error)
            )

        return(
            res.success(
                result.setpos(
                    node.start,
                    node.end
                )
            )
        )