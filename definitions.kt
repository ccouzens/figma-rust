/// [Figma documentation](https://www.figma.com/developers/api#color-type)
@Serializable
data class Color (
	val r: Double,
	val g: Double,
	val b: Double,
	val a: Double
)

/// [Figma documentation](https://www.figma.com/developers/api#vector-type)
@Serializable
data class Vector (
	val x: Double,
	val y: Double
)

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

@Serializable
enum class StyleTypeMapKey(val string: String) {
	@SerialName("fill")
	Fill("fill"),
	@SerialName("fills")
	Fills("fills"),
	@SerialName("text")
	Text("text"),
	@SerialName("grid")
	Grid("grid"),
	@SerialName("effect")
	Effect("effect"),
	@SerialName("stroke")
	Stroke("stroke"),
	@SerialName("strokes")
	Strokes("strokes"),
}

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
