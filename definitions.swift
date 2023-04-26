/*
 Generated by typeshare 1.5.0
 */

import Foundation

/// [Figma documentation](https://www.figma.com/developers/api#color-type)
public struct Color: Codable {
	public let r: Double
	public let g: Double
	public let b: Double
	public let a: Double

	public init(r: Double, g: Double, b: Double, a: Double) {
		self.r = r
		self.g = g
		self.b = b
		self.a = a
	}
}

/// [Figma documentation](https://www.figma.com/developers/api#component-type)
public struct Component: Codable {
	public let key: String
	public let name: String
	public let description: String

	public init(key: String, name: String, description: String) {
		self.key = key
		self.name = name
		self.description = description
	}
}

public enum EffectType: String, Codable {
	case innerShadow = "INNER_SHADOW"
	case dropShadow = "DROP_SHADOW"
	case layerBlur = "LAYER_BLUR"
	case backgroundBlur = "BACKGROUND_BLUR"
}

/// [Figma documentation](https://www.figma.com/developers/api#vector-type)
public struct Vector: Codable {
	public let x: Double
	public let y: Double

	public init(x: Double, y: Double) {
		self.x = x
		self.y = y
	}
}

/// A visual effect such as a shadow or blur
/// 
/// [Figma documentation](https://www.figma.com/developers/api#effect-type)
public struct Effect: Codable {
	/// Type of effect
	public let type: EffectType
	/// Is the effect active?
	public let visible: Bool
	/// The color of the shadow
	public let color: Color?
	/// How far the shadow is projected in the x and y directions
	public let offset: Vector?
	/// How far the shadow spreads
	public let spread: Double?

	public init(type: EffectType, visible: Bool, color: Color?, offset: Vector?, spread: Double?) {
		self.type = type
		self.visible = visible
		self.color = color
		self.offset = offset
		self.spread = spread
	}
}

/// Node type indicates what kind of node you are working with: for example, a FRAME node versus a RECTANGLE node. A node can have additional properties associated with it depending on its node type.
public enum NodeType: String, Codable {
	case document = "DOCUMENT"
	case canvas = "CANVAS"
	case frame = "FRAME"
	case group = "GROUP"
	case vector = "VECTOR"
	case booleanOperation = "BOOLEAN_OPERATION"
	case star = "STAR"
	case line = "LINE"
	case ellipse = "ELLIPSE"
	case regularPolygon = "REGULAR_POLYGON"
	case rectangle = "RECTANGLE"
	case text = "TEXT"
	case slice = "SLICE"
	case component = "COMPONENT"
	case componentSet = "COMPONENT_SET"
	case instance = "INSTANCE"
	case sticky = "STICKY"
	case shapeWithText = "SHAPE_WITH_TEXT"
	case connector = "CONNECTOR"
	case section = "SECTION"
}

public enum PaintType: String, Codable {
	case solid = "SOLID"
	case gradientLinear = "GRADIENT_LINEAR"
	case gradientRadial = "GRADIENT_RADIAL"
	case gradientAngular = "GRADIENT_ANGULAR"
	case gradientDiamond = "GRADIENT_DIAMOND"
	case image = "IMAGE"
}

/// how layer blends with layers below
/// 
/// [Figma documentation](https://www.figma.com/developers/api#blendmode-type)
public enum BlendMode: String, Codable {
	case passThrough = "PASS_THROUGH"
	case normal = "NORMAL"
	case darken = "DARKEN"
	case multiply = "MULTIPLY"
	case linearBurn = "LINEAR_BURN"
	case colorBurn = "COLOR_BURN"
	case lighten = "LIGHTEN"
	case screen = "SCREEN"
	case linearDodge = "LINEAR_DODGE"
	case colorDodge = "COLOR_DODGE"
	case overlay = "OVERLAY"
	case softLight = "SOFT_LIGHT"
	case hardLight = "HARD_LIGHT"
	case difference = "DIFFERENCE"
	case exclusion = "EXCLUSION"
	case hue = "HUE"
	case saturation = "SATURATION"
	case color = "COLOR"
	case luminosity = "LUMINOSITY"
}

/// A solid color, gradient, or image texture that can be applied as fills or strokes
/// 
/// [Figma documentation](https://www.figma.com/developers/api#paint-type)
public struct Paint: Codable {
	public let type: PaintType
	/// Is the paint enabled?
	public let visible: Bool?
	/// Overall opacity of paint (colors within the paint can also have opacity values which would blend with this)
	public let opacity: Double?
	/// Solid color of the paint
	public let color: Color?
	/// How this node blends with nodes behind it in the scene
	public let blend_mode: BlendMode?
	/// This field contains three vectors, each of which are a position in normalized object space (normalized object space is if the top left corner of the bounding box of the object is (0, 0) and the bottom right is (1,1)). The first position corresponds to the start of the gradient (value 0 for the purposes of calculating gradient stops), the second position is the end of the gradient (value 1), and the third handle position determines the width of the gradient. See image examples below:
	public let gradient_handle_positions: [Vector]?

	public init(type: PaintType, visible: Bool?, opacity: Double?, color: Color?, blend_mode: BlendMode?, gradient_handle_positions: [Vector]?) {
		self.type = type
		self.visible = visible
		self.opacity = opacity
		self.color = color
		self.blend_mode = blend_mode
		self.gradient_handle_positions = gradient_handle_positions
	}
}

public enum StrokeAlign: String, Codable {
	/// stroke drawn inside the shape boundary
	case inside = "INSIDE"
	/// stroke drawn outside the shape boundary
	case outside = "OUTSIDE"
	/// stroke drawn centered along the shape boundary
	case center = "CENTER"
}

/// Animation easing curves
/// 
/// [Figma documentation](https://www.figma.com/developers/api#easingtype-type)
public enum EasingType: String, Codable {
	/// Ease in with an animation curve similar to CSS ease-in
	case easeIn = "EASE_IN"
	/// Ease out with an animation curve similar to CSS ease-out
	case easeOut = "EASE_OUT"
	/// Ease in and then out with an animation curve similar to CSS ease-in-out
	case easeInAndOut = "EASE_IN_AND_OUT"
	/// No easing, similar to CSS linear
	case linear = "LINEAR"
	case easeInBack = "EASE_IN_BACK"
	case easeOutBack = "EASE_OUT_BACK"
	case easeInAndOutBack = "EASE_IN_AND_OUT_BACK"
	case customBezier = "CUSTOM_BEZIER"
	case gentle = "GENTLE"
	case quick = "QUICK"
	case bouncy = "BOUNCY"
	case slow = "SLOW"
	case customSpring = "CUSTOM_SPRING"
}

/// [Figma documentation](https://www.figma.com/developers/api#rectangle-type)
public struct Rectangle: Codable {
	public let x: Double?
	public let y: Double?
	public let width: Double?
	public let height: Double?

	public init(x: Double?, y: Double?, width: Double?, height: Double?) {
		self.x = x
		self.y = y
		self.width = width
		self.height = height
	}
}

public enum PrimaryAxisAlignItems: String, Codable {
	case min = "MIN"
	case center = "CENTER"
	case max = "MAX"
	case spaceBetween = "SPACE_BETWEEN"
}

public enum CounterAxisAlignItems: String, Codable {
	case min = "MIN"
	case center = "CENTER"
	case max = "MAX"
	case baseline = "BASELINE"
}

public enum LayoutMode: String, Codable {
	case none = "NONE"
	case horizontal = "HORIZONTAL"
	case vertical = "VERTICAL"
}

public enum StyleTypeMapKey: String, Codable {
	case fill
	case fills
	case text
	case grid
	case effect
	case stroke
	case strokes
}

/// Metadata for character formatting
/// 
/// [Figma documentation](https://www.figma.com/developers/api#typestyle-type)
public struct TypeStyle: Codable {
	/// Font family of text (standard name)
	public let fontFamily: String
	/// Numeric font weight
	public let fontWeight: Double
	/// Font size in px
	public let fontSize: Double
	/// Line height in px
	public let lineHeightPx: Double

	public init(fontFamily: String, fontWeight: Double, fontSize: Double, lineHeightPx: Double) {
		self.fontFamily = fontFamily
		self.fontWeight = fontWeight
		self.fontSize = fontSize
		self.lineHeightPx = lineHeightPx
	}
}

/// [Figma documentation](https://www.figma.com/developers/api#node-types)
public struct Node: Codable {
	/// A string uniquely identifying this node within the document.
	public let id: String
	/// The name given to the node by the user in the tool.
	public let name: String
	/// Whether or not the node is visible on the canvas.
	public let visible: Bool?
	/// The type of the node
	public let type: NodeType
	/// An array of nodes that are direct children of this node
	public let children: [Node]?
	/// Background color of the canvas
	public let backgroundColor: Color?
	/// An array of fill paints applied to the node
	public let fills: [Paint]?
	/// An array of stroke paints applied to the node
	public let strokes: [Paint]?
	/// The weight of strokes on the node
	public let strokeWeight: Double?
	/// Position of stroke relative to vector outline
	public let strokeAlign: StrokeAlign?
	/// Radius of each corner of the node if a single radius is set for all corners
	public let cornerRadius: Double?
	/// Array of length 4 of the radius of each corner of the node, starting in the top left and proceeding clockwise
	public let rectangleCornerRadii: [Double]?
	/// The duration of the prototyping transition on this node (in milliseconds)
	public let transitionDuration: Double?
	/// The easing curve used in the prototyping transition on this node
	public let transitionEasing: EasingType?
	/// Opacity of the node
	public let opacity: Double?
	/// Bounding box of the node in absolute space coordinates
	public let absoluteBoundingBox: Rectangle?
	/// The bounds of the rendered node in the file in absolute space coordinates
	public let absoluteRenderBounds: Rectangle?
	/// Determines how the auto-layout frame’s children should be aligned in the primary axis direction. This property is only applicable for auto-layout frames.
	public let primaryAxisAlignItems: PrimaryAxisAlignItems?
	/// Determines how the auto-layout frame’s children should be aligned in the counter axis direction. This property is only applicable for auto-layout frames.
	public let counterAxisAlignItems: CounterAxisAlignItems?
	/// The distance between children of the frame. Can be negative. This property is only applicable for auto-layout frames.
	public let itemSpacing: Double?
	/// Whether this layer uses auto-layout to position its children.
	public let layoutMode: LayoutMode?
	/// The padding between the left border of the frame and its children. This property is only applicable for auto-layout frames.
	public let paddingLeft: Double?
	/// The padding between the right border of the frame and its children. This property is only applicable for auto-layout frames.
	public let paddingRight: Double?
	/// The padding between the top border of the frame and its children. This property is only applicable for auto-layout frames.
	public let paddingTop: Double?
	/// The padding between the bottom border of the frame and its children. This property is only applicable for auto-layout frames.
	public let paddingBottom: Double?
	/// An array of effects attached to this node
	public let effects: [Effect]?
	/// A mapping of a StyleType to style ID of styles present on this node. The style ID can be used to look up more information about the style in the top-level styles field.
	public let styles: [StyleTypeMapKey: String]?
	/// Text contained within a text box
	public let characters: String?
	/// Style of text including font family and weight
	public let style: TypeStyle?

	public init(id: String, name: String, visible: Bool?, type: NodeType, children: [Node]?, backgroundColor: Color?, fills: [Paint]?, strokes: [Paint]?, strokeWeight: Double?, strokeAlign: StrokeAlign?, cornerRadius: Double?, rectangleCornerRadii: [Double]?, transitionDuration: Double?, transitionEasing: EasingType?, opacity: Double?, absoluteBoundingBox: Rectangle?, absoluteRenderBounds: Rectangle?, primaryAxisAlignItems: PrimaryAxisAlignItems?, counterAxisAlignItems: CounterAxisAlignItems?, itemSpacing: Double?, layoutMode: LayoutMode?, paddingLeft: Double?, paddingRight: Double?, paddingTop: Double?, paddingBottom: Double?, effects: [Effect]?, styles: [StyleTypeMapKey: String]?, characters: String?, style: TypeStyle?) {
		self.id = id
		self.name = name
		self.visible = visible
		self.type = type
		self.children = children
		self.backgroundColor = backgroundColor
		self.fills = fills
		self.strokes = strokes
		self.strokeWeight = strokeWeight
		self.strokeAlign = strokeAlign
		self.cornerRadius = cornerRadius
		self.rectangleCornerRadii = rectangleCornerRadii
		self.transitionDuration = transitionDuration
		self.transitionEasing = transitionEasing
		self.opacity = opacity
		self.absoluteBoundingBox = absoluteBoundingBox
		self.absoluteRenderBounds = absoluteRenderBounds
		self.primaryAxisAlignItems = primaryAxisAlignItems
		self.counterAxisAlignItems = counterAxisAlignItems
		self.itemSpacing = itemSpacing
		self.layoutMode = layoutMode
		self.paddingLeft = paddingLeft
		self.paddingRight = paddingRight
		self.paddingTop = paddingTop
		self.paddingBottom = paddingBottom
		self.effects = effects
		self.styles = styles
		self.characters = characters
		self.style = style
	}
}

public enum StyleType: String, Codable {
	case fill = "FILL"
	case text = "TEXT"
	case effect = "EFFECT"
	case grid = "GRID"
}

/// [Figma documentation](https://www.figma.com/developers/api#style-type)
public struct Style: Codable {
	public let key: String
	public let name: String
	public let description: String
	public let remote: Bool
	public let styleType: StyleType

	public init(key: String, name: String, description: String, remote: Bool, styleType: StyleType) {
		self.key = key
		self.name = name
		self.description = description
		self.remote = remote
		self.styleType = styleType
	}
}

public struct File: Codable {
	public let document: Node
	public let components: [String: Component]
	public let styles: [String: Style]
	public let name: String
	public let schemaVersion: UInt8
	public let version: String

	public init(document: Node, components: [String: Component], styles: [String: Style], name: String, schemaVersion: UInt8, version: String) {
		self.document = document
		self.components = components
		self.styles = styles
		self.name = name
		self.schemaVersion = schemaVersion
		self.version = version
	}
}
