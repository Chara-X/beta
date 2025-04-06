package graphql

import "github.com/graphql-go/graphql"

// [graphql.Schema]
type Schema struct{}

// [graphql.NewSchema]
func NewSchema(config graphql.SchemaConfig) (Schema, error)

type Interface struct{}

// [graphql.NewInterface]
func NewInterface(config graphql.InterfaceConfig) *Interface

// [graphql.Interface.Name]
func (gt *Interface) Name() string

// [graphql.Interface.Description]
func (gt *Interface) Description() string

// [graphql.Interface.Fields]
func (gt *Interface) Fields() map[string]*graphql.FieldDefinition

// [graphql.Object]
type Object struct{}

// [graphql.NewObject]
func NewObject(config graphql.ObjectConfig) *Object

// [graphql.Object.Name]
func (gt *Object) Name() string

// [graphql.Object.Description]
func (gt *Object) Description() string

// [graphql.Object.Interfaces]
func (gt *Object) Interfaces() []*Interface

// [graphql.Object.Fields]
func (gt *Object) Fields() map[string]*graphql.FieldDefinition

// [graphql.NonNull]
type NonNull struct {
	OfType graphql.Type `json:"ofType"`
}

// [graphql.NewNonNull]
func NewNonNull(ofType graphql.Type) *NonNull

// [graphql.NonNull.Name]
func (gl *NonNull) Name() string

// [graphql.NonNull.Description]
func (gl *NonNull) Description() string

// [graphql.Enum]
type Enum struct{}

// [graphql.NewEnum]
func NewEnum(config graphql.EnumConfig) *Enum

// [graphql.Enum.Name]
func (gt *Enum) Name() string

// [graphql.Enum.Description]
func (gt *Enum) Description() string

// [graphql.Enum.Values]
func (gt *Enum) Values() map[string]*graphql.EnumValueDefinition
