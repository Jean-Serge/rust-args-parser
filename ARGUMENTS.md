# Parser

## AST description

+ **program**       :   STRING
+ **options**       :   short-option || long-option
+ **short-option**  :   "-"CHAR [ STRING ... ]
+ **long-option**   :   "--"STRING [ STRING ... ]
+ **argument**      :   STRING

**Arguments** is always at the end of the Input Stream
**Program** is always at the beginning of the Input Stream
