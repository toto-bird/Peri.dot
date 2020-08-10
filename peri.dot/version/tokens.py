TT_INT   = 'INT'
TT_FLOAT = 'FLOAT'

TT_EOL   = 'EOL'
TT_EOF   = 'EOF'

class Token():
    def __init__(self, type_, value=None, start=None, end=None):
        self.type = type_
        self.value = value
        if start:
            self.start = start.copy()
            self.end = start.copy()
            self.end.advance()
        if end:
            self.end = end.copy()

    def __repr__(self):
        if self.value:
            return(f'<{self.type}: {self.value}>')
        else:
            return(f'<{self.type}>')

    def matches(self, type_, value):
        return(self.type == type_ and self.value == value)