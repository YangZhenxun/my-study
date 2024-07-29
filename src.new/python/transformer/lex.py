import ply.lex as lex
import re
from ply.lex import TOKEN

reserved = {
    "if":"IF",
    "while":"WHILE",
    "then":"THEN",
    "else":"ELSE",
    "do":"DO",
    "typedef":"TYPEDEF",
    "typeid":"TYPEID",
    "int":"INT",
    "void":"VOID",
    "bool":"BOOL",
    "for":"FOR",
}

tokens = [
    'NUMBER',
    'PLUS',
    'MINUS',
    'TIMES',
    'DIVIDE',
    'LPAREN',
    'RPAREN',
    'STRING',
    'ID',
]+list(reserved.values())

t_PLUS    = r'\+'
t_MINUS   = r'-'
t_TIMES   = r'\*'
t_DIVIDE  = r'/'
t_LPAREN  = r'\('
t_RPAREN  = r'\)'
t_STRING = r'"'

def t_NUMBER(t):
    r'\d+'
    t.value = int(t.value)
    return t

def t_newline(t):
    r'\n+'
    t.lexer.lineno += len(t.value)

def t_ID(t):
    r'[a-zA-Z_][a-zA-Z_0-9]*'
    t.type = reserved.get(t.value,'ID')
    return t

t_ignore  = ' \t'
t_ignore_COMMENT = '//.*'
t_ignore_COMM = '/\*.*\*/'

def COM(t):
    real = r'/\*.*'
    while True:
        t_newline(t)
        if re.match("\*/",t.value):
            return real+t.value

def my_t(t):
    @TOKEN(COM(t))
    def t_COM(t):
        pass

def t_error(t):
    print("Illegal character '%s'" % t.value[0])
    t.lexer.skip(1)

lexer = lex.lex()
"""while True:
    n = input("Transfromer>")
    lexer.input(n)
    while True:
        tok = lexer.token()
        if not tok:
            break
        print(tok)"""
n = "/* create me \n \
    from the un kown s pace \n \
    */"
lexer.input(n)
while True:
    tok = lexer.token()
    if not tok:
        break
    print(tok)

