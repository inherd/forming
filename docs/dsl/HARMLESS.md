# HARMLESS

> Harmless, a Hardware Architecture Description Language Dedicated to Real-Time Embedded System Simulation

```
format StatusMayUpdate 
  select slice {0}
    case 0 is #noStatus
    case 1 is #useStatus 
  end select
end format
```

```
syntax useOvFlag 
  select
    case #noOv
        −− nothing to write
    case #useOv "o"
  end select 
end syntax
```


```
architecture PPC5516 { 
    device SRUDev : SRU {
        read is GPR.read8 | GPR.read16 |
                GPR.read32 | spr.read
        write is GPR. write8 GPR. write32 | 
                GPR. write16 | | spr . write
        port rs : read (3);
        port rd : write (2) ;
    }
    
    device ALUDev : ALU {
        −− an empty method list means all the methods
        port all;
    } 
}
```


```
pipeline e200z1 maps to PPC5516 {
    stage IFETCH {
        memDev : fetch
    }
    stage DECODE_EA { 
        BPUDev : branch
        SRUDev : rs
        EAUDev : effAddrUnit 
    }
}
```