# AADL

## Two-state error model


Error Model Type [independent]

```
error model independent 
features
  Error_Free: initial error state; 
  Failed: error state;
  Fail: error event
        {Occurrence => poisson λ}; 
  Restart: error event
        {Occurrence => poisson μ}; 
  FailedVisible: out error propagation
        {Occurrence => fixed p}; 
end independent;
```

Error Model Implementation [independent.general]

```
error model implementation
independent.general
transitions
    Error_Free-[Fail]->Failed; 
    Failed-[Restart]->Error_Free; 
    Failed-[out FailedVisible]->Failed;
end independent.general;
```

## System

```
thread software 
features
    Snd: out data port; 
    Receive: in data port; 
    Input: in event data port; 
    Output: out event data port; 
    IAmPrim: out event port;
end software;

thread implementation software.generic 
annex Error_Model {**
    Model => independent.general;
    Guard_Event =>
(Receive[FailedVisible] and self[Error_Free])
                        applies to IAmPrim;
**}
end software.generic;

system HotStandBy 
features
    sysInput: in event data port;
    sysOutput: out event data port; 
end HotStandBy;     

system implementation HotStandBy.checkpoints 
subcomponents
    Comp1: thread software.generic;
    Comp2: thread software.generic; 
connections
    data port Comp1.Snd->Comp2.Receive 
        in modes Comp1Primary;
    data port Comp2.Snd->Comp1.Receive 
        in modes Comp2Primary;
    event data port sysInput->Comp1.Input; 
    event data port sysInput->Comp2.Input; 
    event data port Comp1.Output->sysOutput
        in modes Comp1Primary;
    event data port Comp2.Output->sysOutput
        in modes Comp2Primary; 
modes
    Comp1Primary: initial mode;
    Comp2Primary: mode;
    Comp1Primary -[Comp2.IAmPrim]->Comp2Primary; 
    Comp2Primary -[Comp1.IAmPrim]->Comp1Primary;
end HotStandBy.checkpoints;                   
```