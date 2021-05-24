# ADLARS

1. Component Template 
2. Task Template
3. System Description 
4. System Environment

## Component

```
Component Template Component_Template_One {

interaction themes : { interactionTheme1, interactionTheme2,
                       interactionTheme3};

features:
{ mandatory: {f1, pf2(int a, byte b), f3};
  optional: {f4, f5, f6};
  alternatives: {(f7, f8), (f9, f10, f11)}; 
}

sub-components : {
contents : {


```

## Task


```
input alphabet : {
  ieventName1 :
  {
data : {
       byte : (40 ~ 1500) : "information ...";
       string :     64    : "information ...";
      }
sink component : c1; implications : "description ..."; occurrence : periodic(500); deadline : n/a;
response :
{
         c1 >> {oeventName1};
      }
recovery : {
margin : 2 ms;
action : c3 >> {oeventName2}; }
}
```

## System Description

```
system description ("My_System") {
synchronized TaskTemplate3 task3Inst1();

connect(task1Inst1, task3Inst1)
    using (eventType1);
connect(task1Inst2, task2Inst1)
    using (eventType2);
```

##  System Environmen

- Features
- Event types
- Service definition
- Interaction themes
- Polices

