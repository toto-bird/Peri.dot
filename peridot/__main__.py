from version.catch import *
from version.context            import *
from version.exceptions         import *
from version.lexer              import *
from version.parser             import *
from version.interpreter        import *

@catch
def improvederrormessage():

    ##########################################
    # DEPENDENCIES                           #
    ##########################################

    import sys

    import version.modules.click.src.click    as click
    from   version.modules.colorama.colorama import init, Fore, Style
    init()

    ##########################################
    # LOGO                                   #
    ##########################################

    VERSION = 'Pre-02'

    def logo():
        logolines = [
            f'    _____----_____',
            f'   / \ PERI.DOT / \   Peri.dot - {VERSION}',
            f' _/  /\ __     /\  \_  © 2020 Totobird Creations',
            f'/___/  \__\-,  \ \___\ ',
            f'\‾‾‾\ \  \'-\‾‾\  /‾‾‾/ ',
            f' ‾\  \/     ‾‾ \/  /‾  https://github.com/toto-bird/Peri.dot',
            f'   \ / LANGUAGE \ /',
            f'    ‾‾‾‾‾----‾‾‾‾‾'
        ]
        logolength = 0
        for i in range(len(logolines)):
            j = logolines[i]
            if len(j) > logolength:
                logolength = len(j)
        for i in range(len(logolines)):
            logolines[i] = f' ║ {logolines[i].ljust(logolength, " ")} ║'
        logolines.insert(0, f' ╔{"═" * (logolength + 2)}╗')
        logolines.append(f' ╚{"═" * (logolength + 2)}╝')
        logo = '\n'.join(logolines)
        logo = f'{Fore.GREEN}{Style.BRIGHT}{logo}{Style.RESET_ALL}'
        print(logo, end='\n')
        return(logo)

    ##########################################
    # ARGUMENTS                              #
    ##########################################

    @click.command()
    @click.option('-h', '--help',    is_flag=True)
    @click.option('-v', '--version', is_flag=True)
    @click.option('-r', '--repl',    is_flag=True)
    @click.argument('filename', default='')
    def main(help, version, repl, filename):
        if version:
            logo()


        if help:
            print(f'''{Fore.YELLOW}Usage{Style.RESET_ALL}: {Fore.YELLOW}{Style.BRIGHT}{__file__} [OPTIONS]* [FILE]?{Style.RESET_ALL}

{Fore.BLUE}{Style.BRIGHT}Options:{Style.RESET_ALL}
  {Fore.GREEN}{Style.BRIGHT}-h{Style.RESET_ALL}, {Fore.GREEN}{Style.BRIGHT}--help{Style.RESET_ALL}    - {Fore.GREEN}Display this help message.{Style.RESET_ALL}
  {Fore.GREEN}{Style.BRIGHT}-v{Style.RESET_ALL}, {Fore.GREEN}{Style.BRIGHT}--version{Style.RESET_ALL} - {Fore.GREEN}Display logo and version.{Style.RESET_ALL}
  {Fore.GREEN}{Style.BRIGHT}-r{Style.RESET_ALL}, {Fore.GREEN}{Style.BRIGHT}--repl{Style.RESET_ALL}    - {Fore.GREEN}Enter the repl.{Style.RESET_ALL}''')


        if filename:
            text = ''

            try:
                with open(filename, 'r') as f:
                    text = f.read()

            except Exception as e:
                exc = sys.exc_info()
                print(Cmd_CmdArgumentError(f'{exc[0].__name__}: {str(e)}', 'filename', filename).asstring())
                exit(1)


            symbols = SymbolTable()

            for ln in text.split('\n'):
                lexer = Lexer(filename, ln)
                tokens, error = lexer.maketokens()

                if error:
                    print(error.asstring())
                    exit(1)

                if len(tokens) - 2:
                    parser = Parser(tokens)
                    ast = parser.parse()

                    if ast.error:
                        print(ast.error.asstring())
                        exit(1)

                    interpreter = Interpreter()
                    context = Context('<module>', symbols=symbols)
                    result = interpreter.visit(ast.node, context)

                    if result.error:
                        print(result.error.asstring())
                        exit(1)

                    print(result.value)


        elif not (version or help) or repl:
            print(Cmd_NotSupportedError(f'The repl is not yet supported', 'none', '').asstring())
            exit(1)


    main()
