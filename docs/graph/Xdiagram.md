# Xdiagram

GitHub: https://github.com/andre-santos-pt/xdiagram

## diagram and node

```
diagram ConceptualModel {
  contains ConceptualModel . entities ;
}                   

node Entity rectangle {
  size 200 100 resizable ;
  corner 10;
  
  child label {
      text ”<<abstract >>”;
      position 50% 0;
      size 150 15;
      invisible if Entity.abstract = false;
  }
  
  child label {
    position 50% 15;
    text Entity .name;
    size 150 15;
  }
  
  child hline {
    position 0 30;
    size 0];
  }
  
  child invisible {
    contains Entity . attributes ;
    size 5] 5];
    position 5 30;
    layout vertical ;
  }
}
  
node Attribute label {
 size 100 15;
 text ”− ” Attribute .name ” : ” Attribute . type ;
}
```

## Links

```
link reference Entity . extends { 
  foreground black ;
  decorator 100% triangle {
    foreground black;
    background white;
    size 20 20;
  } 
}
  
link class Association
source Entity.associations target Association.to {

  decorator 0% rhombus {
    size 20 20;
    background black i f Association . type = COMPOSITION;
    background white i f Association . type = AGGREGATION; 
    invisible if Association . type = REFERENCE;
  }

  decorator 50% label {
    text Association .name;
  }
  
  decorator 85% label {
    text Association . cardinality ;
  }
  
  decorator 100% arrow {
    invisible if Association . type <> REFERENCE;
    size 15 15;
  }
}
```

