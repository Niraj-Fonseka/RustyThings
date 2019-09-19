## Common programming concepts

* Default variables are immutable

* There are trade-offs
    -   Mutating an instane in place may be faster than copying returning newly allocated instances 
    -   With smaller data strutures creating new instances writing in a more functional programming style may be easier to reason about. 

* mut isnt allowed to be used with constants. 
    -    Declare constants using the keyword `const` instead of the `let` keyword
