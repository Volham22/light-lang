if version < 600
  syntax clear
elseif exists("b:current_syntax")
  finish
endif

syn keyword     lightConditional    if else
syn keyword     lightStatement      import let struct
syn keyword     lightKeyword        break continue export ptr fn return
syn keyword     lightRepeat         for while loop
syn keyword     lightType           number real bool void string char
syn keyword     lightOperator       addrof deref and or not + - * / % ::
syn keyword     lightBoolean        true false null
syn keyword     lightTodo           TODO FIXME XXX

syn region      lightString         start=+L\="+ skip=+\\\\\|\\"+ end=+"+ contains=lightSpecial,lightSpecialError,Spell
syn match       lightFunction "\zs\(\k\w*\)*\s*\ze("
syn match       lightNumber   '\d\+'
syn match       lightFloat    '\d\+\.\d+'
syn match       lightChar     '\'.\''
syn match       lightComment  "//.*$"

let b:current_syntax = "light"

hi def link      lightStatement       Statement
hi def link      lightFunction        Function
hi def link      lightConditional     Conditional
hi def link      lightRepeat          Repeat
hi def link      lightKeyword         Keyword
hi def link      lightOperator        Operator
hi def link      lightType            Type
hi def link      lightString          String
hi def link      lightNumber          Number
hi def link      lightFloat           Float
hi def link      lightChar            Character
hi def link      lightComment         Comment
hi def link      lightBoolean         Boolean
hi def link      lightTodo            Todo

