/// [Figma documentation](https://www.figma.com/developers/api#color-type)
@Serializable
data class Color (
	val r: Double,
	val g: Double,
	val b: Double,
	val a: Double
)

/// [Figma documentation](https://www.figma.com/developers/api#component-type)
@Serializable
data class Component (
	val key: String,
	val name: String,
	val description: String
)

@Serializable
enum class EffectType(val string: String) {
	@SerialName("INNER_SHADOW")
	InnerShadow("INNER_SHADOW"),
	@SerialName("DROP_SHADOW")
	DropShadow("DROP_SHADOW"),
	@SerialName("LAYER_BLUR")
	LayerBlur("LAYER_BLUR"),
	@SerialName("BACKGROUND_BLUR")
	BackgroundBlur("BACKGROUND_BLUR"),
}

/// [Figma documentation](https://www.figma.com/developers/api#vector-type)
@Serializable
data class Vector (
	val x: Double,
	val y: Double
)

/// A visual effect such as a shadow or blur
/// 
/// [Figma documentation](https://www.figma.com/developers/api#effect-type)
@Serializable
data class Effect (
	/// Type of effect
	val type: EffectType,
	/// Is the effect active?
	val visible: Boolean,
	/// The color of the shadow
	val color: Color? = null,
	/// How far the shadow is projected in the x and y directions
	val offset: Vector? = null,
	/// How far the shadow spreads
	val spread: Double? = null
)

/// Node type indicates what kind of node you are working with: for example, a FRAME node versus a RECTANGLE node. A node can have additional properties associated with it depending on its node type.
@Serializable
enum class NodeType(val string: String) {
	@SerialName("DOCUMENT")
	Document("DOCUMENT"),
	@SerialName("CANVAS")
	Canvas("CANVAS"),
	@SerialName("FRAME")
	Frame("FRAME"),
	@SerialName("GROUP")
	Group("GROUP"),
	@SerialName("VECTOR")
	Vector("VECTOR"),
	@SerialName("BOOLEAN_OPERATION")
	BooleanOperation("BOOLEAN_OPERATION"),
	@SerialName("STAR")
	Star("STAR"),
	@SerialName("LINE")
	Line("LINE"),
	@SerialName("ELLIPSE")
	Ellipse("ELLIPSE"),
	@SerialName("REGULAR_POLYGON")
	RegularPolygon("REGULAR_POLYGON"),
	@SerialName("RECTANGLE")
	Rectangle("RECTANGLE"),
	@SerialName("TEXT")
	Text("TEXT"),
	@SerialName("SLICE")
	Slice("SLICE"),
	@SerialName("COMPONENT")
	Component("COMPONENT"),
	@SerialName("COMPONENT_SET")
	ComponentSet("COMPONENT_SET"),
	@SerialName("INSTANCE")
	Instance("INSTANCE"),
	@SerialName("STICKY")
	Sticky("STICKY"),
	@SerialName("SHAPE_WITH_TEXT")
	ShapeWithText("SHAPE_WITH_TEXT"),
	@SerialName("CONNECTOR")
	Connector("CONNECTOR"),
	@SerialName("SECTION")
	Section("SECTION"),
}

@Serializable
enum class PaintType(val string: String) {
	@SerialName("SOLID")
	Solid("SOLID"),
	@SerialName("GRADIENT_LINEAR")
	GradientLinear("GRADIENT_LINEAR"),
	@SerialName("GRADIENT_RADIAL")
	GradientRadial("GRADIENT_RADIAL"),
	@SerialName("GRADIENT_ANGULAR")
	GradientAngular("GRADIENT_ANGULAR"),
	@SerialName("GRADIENT_DIAMOND")
	GradientDiamond("GRADIENT_DIAMOND"),
	@SerialName("IMAGE")
	Image("IMAGE"),
}

/// how layer blends with layers below
/// 
/// [Figma documentation](https://www.figma.com/developers/api#blendmode-type)
@Serializable
enum class BlendMode(val string: String) {
	@SerialName("PASS_THROUGH")
	PassThrough("PASS_THROUGH"),
	@SerialName("NORMAL")
	Normal("NORMAL"),
	@SerialName("DARKEN")
	Darken("DARKEN"),
	@SerialName("MULTIPLY")
	Multiply("MULTIPLY"),
	@SerialName("LINEAR_BURN")
	LinearBurn("LINEAR_BURN"),
	@SerialName("COLOR_BURN")
	ColorBurn("COLOR_BURN"),
	@SerialName("LIGHTEN")
	Lighten("LIGHTEN"),
	@SerialName("SCREEN")
	Screen("SCREEN"),
	@SerialName("LINEAR_DODGE")
	LinearDodge("LINEAR_DODGE"),
	@SerialName("COLOR_DODGE")
	ColorDodge("COLOR_DODGE"),
	@SerialName("OVERLAY")
	Overlay("OVERLAY"),
	@SerialName("SOFT_LIGHT")
	SoftLight("SOFT_LIGHT"),
	@SerialName("HARD_LIGHT")
	HardLight("HARD_LIGHT"),
	@SerialName("DIFFERENCE")
	Difference("DIFFERENCE"),
	@SerialName("EXCLUSION")
	Exclusion("EXCLUSION"),
	@SerialName("HUE")
	Hue("HUE"),
	@SerialName("SATURATION")
	Saturation("SATURATION"),
	@SerialName("COLOR")
	Color("COLOR"),
	@SerialName("LUMINOSITY")
	Luminosity("LUMINOSITY"),
}

/// A solid color, gradient, or image texture that can be applied as fills or strokes
/// 
/// [Figma documentation](https://www.figma.com/developers/api#paint-type)
@Serializable
data class Paint (
	val type: PaintType,
	/// Is the paint enabled?
	val visible: Boolean? = null,
	/// Overall opacity of paint (colors within the paint can also have opacity values which would blend with this)
	val opacity: Double? = null,
	/// Solid color of the paint
	val color: Color? = null,
	/// How this node blends with nodes behind it in the scene
	val blend_mode: BlendMode? = null,
	/// This field contains three vectors, each of which are a position in normalized object space (normalized object space is if the top left corner of the bounding box of the object is (0, 0) and the bottom right is (1,1)). The first position corresponds to the start of the gradient (value 0 for the purposes of calculating gradient stops), the second position is the end of the gradient (value 1), and the third handle position determines the width of the gradient. See image examples below:
	val gradient_handle_positions: List<Vector>? = null
)

@Serializable
enum class StrokeAlign(val string: String) {
	/// stroke drawn inside the shape boundary
	@SerialName("INSIDE")
	Inside("INSIDE"),
	/// stroke drawn outside the shape boundary
	@SerialName("OUTSIDE")
	Outside("OUTSIDE"),
	/// stroke drawn centered along the shape boundary
	@SerialName("CENTER")
	Center("CENTER"),
}

/// Animation easing curves
/// 
/// [Figma documentation](https://www.figma.com/developers/api#easingtype-type)
@Serializable
enum class EasingType(val string: String) {
	/// Ease in with an animation curve similar to CSS ease-in
	@SerialName("EASE_IN")
	EaseIn("EASE_IN"),
	/// Ease out with an animation curve similar to CSS ease-out
	@SerialName("EASE_OUT")
	EaseOut("EASE_OUT"),
	/// Ease in and then out with an animation curve similar to CSS ease-in-out
	@SerialName("EASE_IN_AND_OUT")
	EaseInAndOut("EASE_IN_AND_OUT"),
	/// No easing, similar to CSS linear
	@SerialName("LINEAR")
	Linear("LINEAR"),
	@SerialName("EASE_IN_BACK")
	EaseInBack("EASE_IN_BACK"),
	@SerialName("EASE_OUT_BACK")
	EaseOutBack("EASE_OUT_BACK"),
	@SerialName("EASE_IN_AND_OUT_BACK")
	EaseInAndOutBack("EASE_IN_AND_OUT_BACK"),
	@SerialName("CUSTOM_BEZIER")
	CustomBezier("CUSTOM_BEZIER"),
	@SerialName("GENTLE")
	Gentle("GENTLE"),
	@SerialName("QUICK")
	Quick("QUICK"),
	@SerialName("BOUNCY")
	Bouncy("BOUNCY"),
	@SerialName("SLOW")
	Slow("SLOW"),
	@SerialName("CUSTOM_SPRING")
	CustomSpring("CUSTOM_SPRING"),
}

/// [Figma documentation](https://www.figma.com/developers/api#rectangle-type)
@Serializable
data class Rectangle (
	val x: Double? = null,
	val y: Double? = null,
	val width: Double? = null,
	val height: Double? = null
)

@Serializable
enum class AxisSizingMode(val string: String) {
	@SerialName("FIXED")
	Fixed("FIXED"),
	@SerialName("AUTO")
	Auto("AUTO"),
}

@Serializable
enum class PrimaryAxisAlignItems(val string: String) {
	@SerialName("MIN")
	Min("MIN"),
	@SerialName("CENTER")
	Center("CENTER"),
	@SerialName("MAX")
	Max("MAX"),
	@SerialName("SPACE_BETWEEN")
	SpaceBetween("SPACE_BETWEEN"),
}

@Serializable
enum class CounterAxisAlignItems(val string: String) {
	@SerialName("MIN")
	Min("MIN"),
	@SerialName("CENTER")
	Center("CENTER"),
	@SerialName("MAX")
	Max("MAX"),
	@SerialName("BASELINE")
	Baseline("BASELINE"),
}

@Serializable
enum class LayoutMode(val string: String) {
	@SerialName("NONE")
	None("NONE"),
	@SerialName("HORIZONTAL")
	Horizontal("HORIZONTAL"),
	@SerialName("VERTICAL")
	Vertical("VERTICAL"),
}

@Serializable
object Styles

@Serializable
enum class TextCase(val string: String) {
	@SerialName("UPPER")
	Upper("UPPER"),
	@SerialName("LOWER")
	Lower("LOWER"),
	@SerialName("TITLE")
	Title("TITLE"),
	@SerialName("SMALL_CAPS")
	SmallCaps("SMALL_CAPS"),
	@SerialName("SMALL_CAPS_FORCED")
	SmallCapsForced("SMALL_CAPS_FORCED"),
}

@Serializable
enum class TextDecoration(val string: String) {
	@SerialName("STRIKETHROUGH")
	Strikethrough("STRIKETHROUGH"),
	@SerialName("UNDERLINE")
	Underline("UNDERLINE"),
}

/// Metadata for character formatting
/// 
/// [Figma documentation](https://www.figma.com/developers/api#typestyle-type)
@Serializable
data class TypeStyle (
	/// Font family of text (standard name)
	val fontFamily: String,
	/// Numeric font weight
	val fontWeight: Double,
	/// Font size in px
	val fontSize: Double,
	/// Text casing applied to the node, default is the original casing
	val textCase: TextCase? = null,
	/// Text decoration applied to the node, default is none
	val textDecoration: TextDecoration? = null,
	/// Line height in px
	val lineHeightPx: Double
)

/// [Figma documentation](https://www.figma.com/developers/api#node-types)
@Serializable
data class Node (
	/// A string uniquely identifying this node within the document.
	val id: String,
	/// The name given to the node by the user in the tool.
	val name: String,
	/// Whether or not the node is visible on the canvas.
	val visible: Boolean? = null,
	/// The type of the node
	val type: NodeType,
	/// An array of nodes that are direct children of this node
	val children: List<Node>? = null,
	/// Background color of the canvas
	val backgroundColor: Color? = null,
	/// An array of fill paints applied to the node
	val fills: List<Paint>? = null,
	/// An array of stroke paints applied to the node
	val strokes: List<Paint>? = null,
	/// The weight of strokes on the node
	val strokeWeight: Double? = null,
	/// Position of stroke relative to vector outline
	val strokeAlign: StrokeAlign? = null,
	/// An array of floating point numbers describing the pattern of dash length and gap lengths that the vector path follows. For example a value of [1, 2] indicates that the path has a dash of length 1 followed by a gap of length 2, repeated.
	val strokeDashes: List<Double>? = null,
	/// Radius of each corner of the node if a single radius is set for all corners
	val cornerRadius: Double? = null,
	/// Array of length 4 of the radius of each corner of the node, starting in the top left and proceeding clockwise
	val rectangleCornerRadii: List<Double>? = null,
	/// The duration of the prototyping transition on this node (in milliseconds)
	val transitionDuration: Double? = null,
	/// The easing curve used in the prototyping transition on this node
	val transitionEasing: EasingType? = null,
	/// Opacity of the node
	val opacity: Double? = null,
	/// Bounding box of the node in absolute space coordinates
	val absoluteBoundingBox: Rectangle? = null,
	/// The bounds of the rendered node in the file in absolute space coordinates
	val absoluteRenderBounds: Rectangle? = null,
	/// Whether the primary axis has a fixed length (determined by the user) or an automatic length (determined by the layout engine). This property is only applicable for auto-layout frames.
	val primaryAxisSizingMode: AxisSizingMode? = null,
	/// Whether the counter axis has a fixed length (determined by the user) or an automatic length (determined by the layout engine). This property is only applicable for auto-layout frames.
	val counterAxisSizingMode: AxisSizingMode? = null,
	/// Determines how the auto-layout frame’s children should be aligned in the primary axis direction. This property is only applicable for auto-layout frames.
	val primaryAxisAlignItems: PrimaryAxisAlignItems? = null,
	/// Determines how the auto-layout frame’s children should be aligned in the counter axis direction. This property is only applicable for auto-layout frames.
	val counterAxisAlignItems: CounterAxisAlignItems? = null,
	/// The distance between children of the frame. Can be negative. This property is only applicable for auto-layout frames.
	val itemSpacing: Double? = null,
	/// Whether this layer uses auto-layout to position its children.
	val layoutMode: LayoutMode? = null,
	/// The padding between the left border of the frame and its children. This property is only applicable for auto-layout frames.
	val paddingLeft: Double? = null,
	/// The padding between the right border of the frame and its children. This property is only applicable for auto-layout frames.
	val paddingRight: Double? = null,
	/// The padding between the top border of the frame and its children. This property is only applicable for auto-layout frames.
	val paddingTop: Double? = null,
	/// The padding between the bottom border of the frame and its children. This property is only applicable for auto-layout frames.
	val paddingBottom: Double? = null,
	/// An array of effects attached to this node
	val effects: List<Effect>? = null,
	/// A mapping of a StyleType to style ID of styles present on this node. The style ID can be used to look up more information about the style in the top-level styles field.
	val styles: Styles? = null,
	/// Text contained within a text box
	val characters: String? = null,
	/// Style of text including font family and weight
	val style: TypeStyle? = null,
	/// This property is applicable only for direct children of auto-layout frames, ignored otherwise. Determines whether a layer should stretch along the parent’s primary axis. A 0 corresponds to a fixed size and 1 corresponds to stretch
	val layoutGrow: Double? = null
)

@Serializable
enum class StyleType(val string: String) {
	@SerialName("FILL")
	Fill("FILL"),
	@SerialName("TEXT")
	Text("TEXT"),
	@SerialName("EFFECT")
	Effect("EFFECT"),
	@SerialName("GRID")
	Grid("GRID"),
}

/// [Figma documentation](https://www.figma.com/developers/api#style-type)
@Serializable
data class Style (
	val key: String,
	val name: String,
	val description: String,
	val remote: Boolean,
	val styleType: StyleType
)

@Serializable
data class File (
	val document: Node,
	val components: HashMap<String, Component>,
	val styles: HashMap<String, Style>,
	val name: String,
	val schemaVersion: UByte,
	val version: String
)

