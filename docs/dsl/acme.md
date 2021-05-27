# ACME

refs: https://acme.able.cs.cmu.edu/docs/language_overview.html



```dsl
System simple_cs = {
    Component client = { Port send-request; };
    Component server = { Port receive-request; };
    Connector rpc = { Roels { caller, callee}};
    Attachments {
        client.send-request to rpc.caller;
        server.receive-request to rpc.callee;
    }
}
```

more 

```dsl
System simple_cs = {
  Component client = {
    Port send-request;
    Property Aesop-style : style-id = client-server;
    Property UniCon-style : style-id = client-server;
    Property source-code : external = "CODE-LIB/client.c";
  }
  Component server = {
    Port receive-request;
    Property idempotence : boolean = true;
    Property max-concurrent-clients : integer = 1;
    source-code : external = "CODE-LIB/server.c";
  }
  Connector rpc = {
    Role caller;
    Role callee;
    Property asynchronous : boolean = true;
    max-roles : integer = 2;
    protocol : Wright = " ... ";
  }
  Attachment client.send-request to rpc.caller;
  Attachment server.receive-request to rpc.callee;
}
```

dsl 2

```
exists  client, server, rpc |
  component(client) ^
  component(server) ^
  connector(rpc) ^
  attached(client.send-request,rpc.caller) ^
  attached(server.receive-request,rpc.callee) ^
  client != server ^ 
  server != rpc ^ 
  client != rpc ^
  (for all y:component (y) =>
                  y = client | y = server) ^
  (for all y:connector(y) => y = rpc) ^
  (for all p,q: attached(p,q) =>
       (p=client.send-request ^ q=rpc.caller)
     | (p=server.receive-request ^ q=rpc.callee))
```

## Syntax

refs: http://acme.able.cs.cmu.edu/html/ArmaniParser.html

```bnf
parse_AcmeDesign 	::= 	( <IMPORT> ( Filename ";" | <STRING_LITERAL> ";" ) )* ( TypeDeclaration | FamilyDeclaration | DesignAnalysisDeclaration | PropertyDeclaration | PropertiesBlock | SystemDeclaration )* <EOF>
Filename 	::= 	( "$" | "%" )? <IDENTIFIER> ( ( ( "." | ":" | "-" | "+" | "\\" | "\\\\" | "/" | "$" | "%" ) )+ <IDENTIFIER> )*
FamilyDeclaration 	::= 	( <FAMILY> | <STYLE> ) <IDENTIFIER> ( ";" | ( "=" FamilyBody ( ";" )? ) | ( <EXTENDS> lookup_SystemTypeByName ( "," lookup_SystemTypeByName )* <WITH> FamilyBody ( ";" )? ) )
FamilyBody 	::= 	"{" "}"
	| 	"{" ( TypeDeclaration | SystemStructure )+ "}"
TypeDeclaration 	::= 	ElementTypeDeclaration
	| 	PropertyTypeDeclaration
ElementTypeDeclaration 	::= 	ElementProtoTypeDeclaration
	| 	ComponentTypeDeclaration
	| 	GroupTypeDeclaration
	| 	ConnectorTypeDeclaration
	| 	PortTypeDeclaration
	| 	RoleTypeDeclaration
ElementProtoTypeDeclaration 	::= 	( <ELEMENT> <TYPE> <IDENTIFIER> ( "=" parse_ElementProtoTypeDescription ( ";" )? | ";" ) | <ELEMENT> <TYPE> <IDENTIFIER> <EXTENDS> lookup_ComponentTypeByName ( "," lookup_ComponentTypeByName )* <WITH> parse_ElementProtoTypeDescription ( ";" )? )
ComponentTypeDeclaration 	::= 	( <COMPONENT> <TYPE> <IDENTIFIER> ( "=" parse_ComponentDescription ( ";" )? | ";" ) | <COMPONENT> <TYPE> <IDENTIFIER> <EXTENDS> lookup_ComponentTypeByName ( "," lookup_ComponentTypeByName )* <WITH> parse_ComponentDescription ( ";" )? )
GroupTypeDeclaration 	::= 	( <GROUP> <TYPE> <IDENTIFIER> ( "=" parse_GroupDescription ( ";" )? | ";" ) | <GROUP> <TYPE> <IDENTIFIER> <EXTENDS> lookup_GroupTypeByName ( "," lookup_GroupTypeByName )* <WITH> parse_GroupDescription ( ";" )? )
ConnectorTypeDeclaration 	::= 	( <CONNECTOR> <TYPE> <IDENTIFIER> ( "=" parse_ConnectorDescription ( ";" )? | ";" ) | <CONNECTOR> <TYPE> <IDENTIFIER> <EXTENDS> lookup_ConnectorTypeByName ( "," lookup_ConnectorTypeByName )* <WITH> parse_ConnectorDescription ( ";" )? )
PortTypeDeclaration 	::= 	( <PORT> <TYPE> <IDENTIFIER> ( "=" parse_PortDescription ( ";" )? | ";" ) | <PORT> <TYPE> <IDENTIFIER> <EXTENDS> lookup_PortTypeByName ( "," lookup_PortTypeByName )* <WITH> parse_PortDescription ( ";" )? )
RoleTypeDeclaration 	::= 	( <ROLE> <TYPE> <IDENTIFIER> ( "=" parse_RoleDescription ( ";" )? | ";" ) | <ROLE> <TYPE> <IDENTIFIER> <EXTENDS> lookup_RoleTypeByName ( "," lookup_RoleTypeByName )* <WITH> parse_RoleDescription ( ";" )? )
lookup_SystemTypeByName 	::= 	<IDENTIFIER>
lookup_ComponentTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_GroupTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_ConnectorTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_PortTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_RoleTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_PropertyTypeByName 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER>
lookup_arbitraryTypeByName 	::= 	( PropertyTypeDescription | <SYSTEM> | <COMPONENT> | <GROUP> | <CONNECTOR> | <PORT> | <ROLE> | <PROPERTY> | <REPRESENTATION> | NonPropertySetTypeExpression )
SystemDeclaration 	::= 	<SYSTEM> <IDENTIFIER> ( ":" lookup_SystemTypeByName ( "," lookup_SystemTypeByName )* )? ( "=" SystemBody ( ";" )? | ";" )
SystemBody 	::= 	( <NEW> lookup_SystemTypeByName ( "," lookup_SystemTypeByName )* | "{" "}" | "{" ( SystemStructure )+ "}" ) ( <EXTENDED> <WITH> SystemBody )?
SystemStructure 	::= 	ComponentDeclaration
	| 	ComponentsBlock
	| 	GroupDeclaration
	| 	ConnectorDeclaration
	| 	ConnectorsBlock
	| 	PortDeclaration
	| 	PortsBlock
	| 	RoleDeclaration
	| 	RolesBlock
	| 	PropertyDeclaration
	| 	PropertiesBlock
	| 	AttachmentsDeclaration
	| 	RepresentationDeclaration
	| 	DesignAnalysisDeclaration
	| 	parse_DesignRule
parse_ElementProtoTypeDescription 	::= 	"{" ( PropertyDeclaration | PropertiesBlock | RepresentationDeclaration )* "}"
GroupDeclaration 	::= 	<GROUP> <IDENTIFIER> ( ":" lookup_GroupTypeByName ( "," lookup_GroupTypeByName )* )? ( "=" parse_GroupDescription ";" | ";" )
parse_GroupDescription 	::= 	( <NEW> lookup_GroupTypeByName ( "," lookup_GroupTypeByName )* | "{" ( MembersBlock | PropertyDeclaration | PropertiesBlock | parse_DesignRule )* "}" ) ( <EXTENDED> <WITH> parse_GroupDescription )?
ComponentDeclaration 	::= 	<COMPONENT> <IDENTIFIER> ( ":" lookup_ComponentTypeByName ( "," lookup_ComponentTypeByName )* )? ( "=" parse_ComponentDescription ";" | ";" )
ComponentsBlock 	::= 	<COMPONENTS> "{" ( <IDENTIFIER> ( ":" lookup_ComponentTypeByName ( "," lookup_ComponentTypeByName )* )? ( "=" parse_ComponentDescription ";" | ";" ) )* "}" ( ";" )?
parse_ComponentDescription 	::= 	( <NEW> lookup_ComponentTypeByName ( "," lookup_ComponentTypeByName )* | "{" ( PortDeclaration | PortsBlock | PropertyDeclaration | PropertiesBlock | RepresentationDeclaration | parse_DesignRule )* "}" ) ( <EXTENDED> <WITH> parse_ComponentDescription )?
ConnectorDeclaration 	::= 	<CONNECTOR> <IDENTIFIER> ( ":" lookup_ConnectorTypeByName ( "," lookup_ConnectorTypeByName )* )? ( "=" parse_ConnectorDescription ";" | ";" )
ConnectorsBlock 	::= 	<CONNECTORS> "{" ( <IDENTIFIER> ( ":" lookup_ConnectorTypeByName ( "," lookup_ConnectorTypeByName )* )? ( "=" parse_ConnectorDescription ";" | ";" ) )* "}" ( ";" )?
parse_ConnectorDescription 	::= 	( <NEW> lookup_ConnectorTypeByName ( "," lookup_ConnectorTypeByName )* | "{" ( RoleDeclaration | RolesBlock | PropertyDeclaration | PropertiesBlock | RepresentationDeclaration | parse_DesignRule )* "}" ) ( <EXTENDED> <WITH> parse_ConnectorDescription )?
PortDeclaration 	::= 	<PORT> <IDENTIFIER> ( ":" lookup_PortTypeByName ( "," lookup_PortTypeByName )* )? ( "=" parse_PortDescription ";" | ";" )
PortsBlock 	::= 	<PORTS> "{" ( <IDENTIFIER> ( ":" lookup_PortTypeByName ( "," lookup_PortTypeByName )* )? ( "=" parse_PortDescription ";" | ";" ) )* "}" ( ";" )?
parse_PortDescription 	::= 	( <NEW> lookup_PortTypeByName ( "," lookup_PortTypeByName )* | "{" ( PropertyDeclaration | PropertiesBlock | RepresentationDeclaration | parse_DesignRule )* "}" ) ( <EXTENDED> <WITH> parse_PortDescription )?
RoleDeclaration 	::= 	<ROLE> <IDENTIFIER> ( ":" lookup_RoleTypeByName ( "," lookup_RoleTypeByName )* )? ( "=" parse_RoleDescription ";" | ";" )
MembersBlock 	::= 	<MEMBERS> "{" ( QualifiedReference ( ";" ) )* "}" ( ";" )?
QualifiedReference 	::= 	<IDENTIFIER> ( ( "." <IDENTIFIER> ) )*
RolesBlock 	::= 	<ROLES> "{" ( <IDENTIFIER> ( ":" lookup_RoleTypeByName ( "," lookup_RoleTypeByName )* )? ( "=" parse_RoleDescription ";" | ";" ) )* "}" ( ";" )?
parse_RoleDescription 	::= 	( <NEW> lookup_RoleTypeByName ( "," lookup_RoleTypeByName )* | "{" ( PropertyDeclaration | PropertiesBlock | RepresentationDeclaration | parse_DesignRule )* "}" ) ( <EXTENDED> <WITH> parse_RoleDescription )?
AttachmentsDeclaration 	::= 	( ( <ATTACHMENTS> "{" ( <IDENTIFIER> "." <IDENTIFIER> "to" <IDENTIFIER> "." <IDENTIFIER> ( "{" ( PropertyDeclaration | PropertiesBlock )* "}" )? ";" )* "}" ( ";" )? ) | ( <ATTACHMENT> <IDENTIFIER> "." <IDENTIFIER> "to" <IDENTIFIER> "." <IDENTIFIER> ( "{" ( PropertyDeclaration | PropertiesBlock )* "}" )? ";" ) )
PropertyDeclaration 	::= 	<PROPERTY> parse_PropertyDescription ";"
PropertiesBlock 	::= 	<PROPERTIES> "{" ( parse_PropertyDescription ( ";" parse_PropertyDescription | ";" )* )? "}" ( ";" )?
parse_PropertyDescription 	::= 	( <PROPERTY> )? <IDENTIFIER> ( ":" PropertyTypeDescription )? ( "=" PropertyValueDeclaration )? ( <PROPBEGIN> parse_PropertyDescription ( ";" parse_PropertyDescription | ";" )* <PROPEND> | <PROPBEGIN> <PROPEND> )?
PropertyTypeDeclaration 	::= 	<PROPERTY> <TYPE> <IDENTIFIER> ( "=" ( <INT> ";" | <FLOAT> ";" | <STRING> ";" | <BOOLEAN> ";" | <ENUM> ( "{" <IDENTIFIER> ( "," <IDENTIFIER> )* "}" )? ";" | <SET> ( "{" "}" )? ";" | <SET> "{" PropertyTypeDescription "}" ";" | <SEQUENCE> ( "<" ">" )? ";" | <SEQUENCE> "<" PropertyTypeDescription ">" ";" | <RECORD> "[" parse_RecordFieldDescription ( ";" parse_RecordFieldDescription | ";" )* "]" ";" | <RECORD> ( "[" "]" )? ";" | <IDENTIFIER> ";" ) )
PropertyTypeDescription 	::= 	<ANY>
	| 	<INT>
	| 	<FLOAT>
	| 	<STRING>
	| 	<BOOLEAN>
	| 	<SET> ( "{" ( PropertyTypeDescription )? "}" )?
	| 	<SEQUENCE> ( "<" ( PropertyTypeDescription )? ">" )?
	| 	<RECORD> "[" parse_RecordFieldDescription ( ";" parse_RecordFieldDescription | ";" )* "]"
	| 	<RECORD> ( "[" "]" )?
	| 	<ENUM> ( "{" <IDENTIFIER> ( "," <IDENTIFIER> )* "}" )?
	| 	<ENUM> ( "{" "}" )?
	| 	lookup_PropertyTypeByName
parse_RecordFieldDescription 	::= 	<IDENTIFIER> ( "," <IDENTIFIER> )* ( ":" PropertyTypeDescription )?
PropertyValueDeclaration 	::= 	<INTEGER_LITERAL>
	| 	<FLOATING_POINT_LITERAL>
	| 	<STRING_LITERAL>
	| 	<FALSE>
	| 	<TRUE>
	| 	AcmeSetValue
	| 	AcmeSequenceValue
	| 	AcmeRecordValue
	| 	<IDENTIFIER>
AcmeSetValue 	::= 	"{" "}"
	| 	"{" PropertyValueDeclaration ( "," PropertyValueDeclaration )* "}"
AcmeSequenceValue 	::= 	"<" ">"
	| 	"<" PropertyValueDeclaration ( "," PropertyValueDeclaration )* ">"
AcmeRecordValue 	::= 	( "[" RecordFieldValue ( ";" RecordFieldValue | ";" )* "]" | "[" "]" )
RecordFieldValue 	::= 	<IDENTIFIER> ( ":" PropertyTypeDescription )? "=" PropertyValueDeclaration
RepresentationDeclaration 	::= 	<REPRESENTATION> ( <IDENTIFIER> "=" )? "{" SystemDeclaration ( BindingsMapDeclaration )? "}" ( ";" )?
BindingsMapDeclaration 	::= 	<BINDINGS> "{" ( BindingDeclaration )* "}" ( ";" )?
BindingDeclaration 	::= 	( <IDENTIFIER> "." )? <IDENTIFIER> "to" ( <IDENTIFIER> "." )? <IDENTIFIER> ( "{" ( PropertyDeclaration | PropertiesBlock )* "}" )? ";"
DesignAnalysisDeclaration 	::= 	( ( <EXTERNAL> ( <DESIGN> )? <ANALYSIS> <IDENTIFIER> "(" FormalParams ")" ":" ( PropertyTypeDescription | <COMPONENT> | <GROUP> | <CONNECTOR> | <PORT> | <ROLE> | <SYSTEM> | <ELEMENT> | <TYPE> ) "=" JavaMethodCallExpr ";" ) | ( ( <DESIGN> )? <ANALYSIS> <IDENTIFIER> "(" FormalParams ")" ":" ( PropertyTypeDescription | <COMPONENT> | <GROUP> | <CONNECTOR> | <PORT> | <ROLE> | <SYSTEM> | <ELEMENT> | <TYPE> ) "=" DesignRuleExpression ";" ) )
parse_DesignRule 	::= 	( <DESIGN> )? ( <INVARIANT> | <HEURISTIC> ) DesignRuleExpression ( <PROPBEGIN> parse_PropertyDescription ( ";" parse_PropertyDescription | ";" )* <PROPEND> )? ";"
DesignRuleExpression 	::= 	QuantifiedExpression
	| 	BooleanExpression
QuantifiedExpression 	::= 	( ( <FORALL> | <EXISTS> ( <UNIQUE> )? ) <IDENTIFIER> ( ( ":" | <SET_DECLARE> ) ( Type | lookup_arbitraryTypeByName ) )? <IN> ( SetExpression | Reference ) "|" DesignRuleExpression )
BooleanExpression 	::= 	OrExpression ( <AND> OrExpression )*
OrExpression 	::= 	ImpliesExpression ( <OR> ImpliesExpression )*
ImpliesExpression 	::= 	IffExpression ( <IMPLIES> IffExpression )*
IffExpression 	::= 	EqualityExpression ( <IFF> EqualityExpression )*
EqualityExpression 	::= 	RelationalExpression ( <EQ> RelationalExpression | <NE> RelationalExpression )*
RelationalExpression 	::= 	AdditiveExpression ( "<" AdditiveExpression | ">" AdditiveExpression | <LE> AdditiveExpression | <GE> AdditiveExpression )*
AdditiveExpression 	::= 	MultiplicativeExpression ( <PLUS> MultiplicativeExpression | <MINUS> MultiplicativeExpression )*
MultiplicativeExpression 	::= 	UnaryExpression ( <STAR> UnaryExpression | <SLASH> UnaryExpression | <REM> UnaryExpression )*
UnaryExpression 	::= 	<BANG> UnaryExpression
	| 	<MINUS> UnaryExpression
	| 	PrimitiveExpression
PrimitiveExpression 	::= 	"(" DesignRuleExpression ")"
	| 	LiteralConstant
	| 	Reference
	| 	SetExpression
Reference 	::= 	<IDENTIFIER> ( ( "." <IDENTIFIER> ) | ( "." <TYPE> ) | ( "." <COMPONENTS> ) | ( "." <CONNECTORS> ) | ( "." <PORTS> ) | ( "." <ROLES> ) | ( "." <MEMBERS> ) | ( "." <PROPERTIES> ) | ( "." <REPRESENTATIONS> ) | ( "." <ATTACHEDPORTS> ) | ( "." <ATTACHEDROLES> ) )* ( "(" ActualParams ")" )?
JavaMethodCallExpr 	::= 	<IDENTIFIER> ( "." <IDENTIFIER> )* "(" ActualParams ")"
LiteralConstant 	::= 	( <INTEGER_LITERAL> )
	| 	( <FLOATING_POINT_LITERAL> )
	| 	( <STRING_LITERAL> )
	| 	( <TRUE> )
	| 	( <FALSE> )
	| 	( <COMPONENT> )
	| 	( <GROUP> )
	| 	( <CONNECTOR> )
	| 	( <PORT> )
	| 	( <ROLE> )
	| 	( <SYSTEM> )
	| 	( <ELEMENT> )
	| 	( <PROPERTY> )
	| 	( <INT> )
	| 	( <FLOAT> )
	| 	( <STRING> )
	| 	( <BOOLEAN> )
	| 	( <ENUM> )
	| 	( <SET> )
	| 	( <SEQUENCE> )
	| 	( <RECORD> )
ActualParams 	::= 	( ActualParam ( "," ActualParam )* )?
FormalParams 	::= 	( FormalParam ( "," FormalParam )* )?
ActualParam 	::= 	DesignRuleExpression
FormalParam 	::= 	<IDENTIFIER> ( "," <IDENTIFIER> )* ":" ( <ELEMENT> | <SYSTEM> | <COMPONENT> | <CONNECTOR> | <PORT> | <ROLE> | <TYPE> | <PROPERTY> | <REPRESENTATION> | <ANY> | NonPropertySetTypeExpression | PropertyTypeDescription )
NonPropertySetTypeExpression 	::= 	<SET> "{" ( <ELEMENT> | <SYSTEM> | <COMPONENT> | <CONNECTOR> | <PORT> | <ROLE> | <TYPE> | <PROPERTY> | <REPRESENTATION> | <ANY> ) "}"
SetExpression 	::= 	( LiteralSet | SetConstructor )
LiteralSet 	::= 	( "{" "}" | "{" ( LiteralConstant | Reference ) ( "," ( LiteralConstant | Reference ) )* "}" )
SetConstructor 	::= 	( "{" <SELECT> <IDENTIFIER> ( ":" lookup_arbitraryTypeByName )? <IN> ( SetExpression | Reference ) "|" DesignRuleExpression "}" | ( "{" <COLLECT> <IDENTIFIER> "." <IDENTIFIER> ":" lookup_arbitraryTypeByName "." lookup_arbitraryTypeByName <IN> ( SetExpression | Reference ) "|" DesignRuleExpression "}" ) )
RecordType 	::= 	<RECORD> "[" RecordItem ( "," RecordItem )* "]"
RecordItem 	::= 	<IDENTIFIER> ":" Type
SetType 	::= 	<SET> "{" Type "}"
SequenceType 	::= 	<SEQUENCE> "{" Type "}"
Signature 	::= 	Type "<->" Type
Type 	::= 	( <IDENTIFIER> ( "." <IDENTIFIER> )* )
PrimitiveType 	::= 	<COMPONENT>
	| 	<GROUP>
	| 	<CONNECTOR>
	| 	<PORT>
	| 	<ROLE>
	| 	<SYSTEM>
Element 	::= 	( <IDENTIFIER> ( "." <IDENTIFIER> )* )
	| 	CompoundElement
CompoundElement 	::= 	Set
	| 	Record
	| 	Sequence
Set 	::= 	"{" Element ( "," Element )* "}"
Record 	::= 	"[" <IDENTIFIER> "=" Element ( "," <IDENTIFIER> "=" Element )* "]"
Sequence 	::= 	"<" Element ( "," Element )* ">"
```