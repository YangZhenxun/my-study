import ast
import pprint

class FuncVisitor(ast.NodeVisitor):
    def __init__(self):
        self.functions = []

    def visit_FunctionDef(self, node):
        func_name = node.name
        func_args = [arg.arg for arg in node.args.args]
        func_defaults = [None] * (len(func_args) - len(node.args.defaults)) + node.args.defaults
        func_code = ast.unparse(node.body).strip()
        self.functions.append({
            'name': func_name,
            'args': func_args,
            'defaults': func_defaults,
            'code': func_code,
        })

with open('your_file.py', 'r') as f:
    source_code = f.read()
    tree = ast.parse(source_code)

visitor = FuncVisitor()
visitor.visit(tree)

for func in visitor.functions:
    print(f"Function name: {func['name']}")
    print(f"Function args: {func['args']}")
    print(f"Function defaults: {func['defaults']}")
    print(f"Function code:\n{func['code']}\n")
    pprint.pprint(func)
    pprint.pprint(ast.dump(tree))