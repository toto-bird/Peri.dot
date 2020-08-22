##########################################
# TOKENS                                 #
##########################################

TT_INT         = 'INT'
TT_FLOAT       = 'FLOAT'
TT_STRING      = 'STRING'

TT_PLUS        = 'PLUS'
TT_DASH        = 'MINUS'
TT_ASTRISK     = 'TIMES'
TT_FSLASH      = 'DIVBY'
TT_CARAT       = 'RAISED'

TT_LPAREN      = 'LPAREN'
TT_RPAREN      = 'RPAREN'
TT_LCURLY      = 'LCURLY'
TT_RCURLY      = 'RCURLY'
TT_LSQUARE     = 'LSQUARE'
TT_RSQUARE     = 'RSQUARE'

TT_EQUALS      = 'EQUALS'
TT_COMMA       = 'COMMA'
TT_COLON       = 'COLON'

TT_EQEQUALS    = 'EQEQUALS'
TT_BANGEQUALS  = 'BANGEQUALS'
TT_LESSTHAN    = 'LESSTHAN'
TT_LTEQUALS    = 'LTEQUALS'
TT_GREATERTHAN = 'GREATERTHAN'
TT_GTEQUALS    = 'GTEQUALS'

TT_KEYWORD     = 'KEYWORD'
TT_IDENTIFIER  = 'IDENTIFIER'

TT_EOL         = 'EOL'
TT_EOF         = 'EOF'

##########################################
# TOKEN OBJECT                           #
##########################################

class Token():
    def __init__(self, type_, value=None, start=None, end=None):
        self.type = type_
        self.value = value
        if start:
            self.start = start.copy()

        if end:
            self.end = end.copy()
        else:
            self.end = start.copy()
            self.end.advance()

    def __repr__(self):
        if isinstance(self.value, str):
            value = f'"{self.value}"'
        else:
            value = self.value

        if self.value:
            return(f'<{self.type}: {value}>')
        else:
            return(f'<{self.type}>')

    def matches(self, type_, value):
        return(self.type == type_ and self.value == value)
