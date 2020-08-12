##########################################
# DEPENDENCIES                           #
##########################################

from .exceptions import *

##########################################
# CONSTANTS                              #
##########################################

TYPES = {
    'invalid'      : 'Invalid',
    'nonetype'     : 'Null',
    'integer'      : 'Int',
    'floatingpoint': 'Float',
    'string'       : 'Str',
    'boolean'      : 'Bool'
}

##########################################
# TYPES                                  #
##########################################

class TypeObj():
    def __init__(self, value, type_=TYPES['invalid']):
        self.value = value
        self.type  = type_

        self.setpos()
        self.setcontext()

    def setpos(self, start=None, end=None):
        self.start = start
        self.end   = end

        return(self)

    def setcontext(self, context=None):
        self.context = context

        return(self)



    def add(self, other):
        return((None, Exc_TypeError(f'{self.type} can not be added to', self.start, self.end, self.context)))
    def subtract(self, other):
        return((None, Exc_TypeError(f'{self.type} can not be subtracted from', self.start, self.end, self.context)))
    def multiply(self, other):
        return((None, Exc_TypeError(f'{self.type} can not be multiplied', self.start, self.end, self.context)))
    def divide(self, other):
        return((None, Exc_TypeError(f'{self.type} can not be divided', self.start, self.end, self.context)))
    def raised(self, other):
        return((None, Exc_TypeError(f'{self.type} can not be raised', self.start, self.end, self.context)))

    def __repr__(self):
        return(f'{self.value}')



class NullType(TypeObj):
    def __init__(self):
        super().__init__(None, type_=TYPES['nonetype'])

    def __repr__(self):
        return(f'Null')



class IntType(TypeObj):
    def __init__(self, value):
        super().__init__(value, type_=TYPES['integer'])


    def add(self, other):
        if isinstance(other, IntType):
            return((
                IntType(self.value + other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{other.type} can not be added to {self.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def subtract(self, other):
        if isinstance(other, IntType):
            return((
                IntType(self.value - other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{other.type} can not be subtracted from {self.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def multiply(self, other):
        if isinstance(other, IntType):
            return((
                IntType(self.value * other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be multiplied by {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def divide(self, other):
        if isinstance(other, IntType):
            if other.value == 0:
                return((
                    None,
                    Exc_ValueError(
                        f'Division by zero',
                        other.start, other.end,
                        self.context
                    )
                ))

            return((
                IntType(int(self.value / other.value))
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be divided by {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def raised(self, other):
        if isinstance(other, IntType):
            return((
                IntType(
                    pow(
                        self.value,
                        other.value
                    )
                )
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be raised to {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))



class FloatType(TypeObj):
    def __init__(self, value):
        super().__init__(value, type_=TYPES['floatingpoint'])


    def add(self, other):
        if isinstance(other, FloatType):
            return((
                FloatType(self.value + other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{other.type} can not be added to {self.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def subtract(self, other):
        if isinstance(other, FloatType):
            return((
                FloatType(self.value - other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{other.type} can not be subtracted from {self.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def multiply(self, other):
        if isinstance(other, FloatType):
            return((
                FloatType(self.value * other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be multiplied by {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def divide(self, other):
        if isinstance(other, FloatType):
            if other.value == 0:
                return((
                    None,
                    Exc_ValueError(
                        f'Division by zero',
                        other.start, other.end,
                        self.context
                    )
                ))

            return((
                FloatType(self.value / other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be divided by {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))

    def raised(self, other):
        if isinstance(other, FloatType):
            return((
                FloatType(self.value ^ other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{self.type} can not be raised to {other.type}',
                    other.start, other.end,
                    self.context
                )
            ))




class StringType(TypeObj):
    def __init__(self, value):
        super().__init__(value, type_=TYPES['string'])


    def add(self, other):
        if isinstance(other, StringType):
            return((
                StringType(self.value + other.value)
                    .setcontext(self.context),
                None
            ))
        else:
            return((
                None,
                Exc_TypeError(
                    f'{other.type} can not be added to {self.type}',
                    other.start, other.end,
                    self.context
                )
            ))




class BooleanType(TypeObj):
    def __init__(self, value):
        if not isinstance(value, bool):
            raise TypeError(f'Internal Error: Non boolean value receievd ({type(value).__name__})')
        super().__init__(value, type_=TYPES['boolean'])