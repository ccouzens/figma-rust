/*
 Generated by typeshare 1.5.0
*/

/** [Figma documentation](https://www.figma.com/developers/api#color-type) */
export interface Color {
	r: number;
	g: number;
	b: number;
	a: number;
}

/** [Figma documentation](https://www.figma.com/developers/api#component-type) */
export interface Component {
	key: string;
	name: string;
	description: string;
}

export enum EffectType {
	InnerShadow = "INNER_SHADOW",
	DropShadow = "DROP_SHADOW",
	LayerBlur = "LAYER_BLUR",
	BackgroundBlur = "BACKGROUND_BLUR",
}

/** [Figma documentation](https://www.figma.com/developers/api#vector-type) */
export interface Vector {
	x: number;
	y: number;
}

/**
 * A visual effect such as a shadow or blur
 * 
 * [Figma documentation](https://www.figma.com/developers/api#effect-type)
 */
export interface Effect {
	/** Type of effect */
	type: EffectType;
	/** Is the effect active? */
	visible: boolean;
	/** The color of the shadow */
	color?: Color;
	/** How far the shadow is projected in the x and y directions */
	offset?: Vector;
	/** How far the shadow spreads */
	spread?: number;
}

/** Node type indicates what kind of node you are working with: for example, a FRAME node versus a RECTANGLE node. A node can have additional properties associated with it depending on its node type. */
export enum NodeType {
	Document = "DOCUMENT",
	Canvas = "CANVAS",
	Frame = "FRAME",
	Group = "GROUP",
	Vector = "VECTOR",
	BooleanOperation = "BOOLEAN_OPERATION",
	Star = "STAR",
	Line = "LINE",
	Ellipse = "ELLIPSE",
	RegularPolygon = "REGULAR_POLYGON",
	Rectangle = "RECTANGLE",
	Text = "TEXT",
	Slice = "SLICE",
	Component = "COMPONENT",
	ComponentSet = "COMPONENT_SET",
	Instance = "INSTANCE",
	Sticky = "STICKY",
	ShapeWithText = "SHAPE_WITH_TEXT",
	Connector = "CONNECTOR",
	Section = "SECTION",
}

export enum PaintType {
	Solid = "SOLID",
	GradientLinear = "GRADIENT_LINEAR",
	GradientRadial = "GRADIENT_RADIAL",
	GradientAngular = "GRADIENT_ANGULAR",
	GradientDiamond = "GRADIENT_DIAMOND",
	Image = "IMAGE",
}

/**
 * how layer blends with layers below
 * 
 * [Figma documentation](https://www.figma.com/developers/api#blendmode-type)
 */
export enum BlendMode {
	PassThrough = "PASS_THROUGH",
	Normal = "NORMAL",
	Darken = "DARKEN",
	Multiply = "MULTIPLY",
	LinearBurn = "LINEAR_BURN",
	ColorBurn = "COLOR_BURN",
	Lighten = "LIGHTEN",
	Screen = "SCREEN",
	LinearDodge = "LINEAR_DODGE",
	ColorDodge = "COLOR_DODGE",
	Overlay = "OVERLAY",
	SoftLight = "SOFT_LIGHT",
	HardLight = "HARD_LIGHT",
	Difference = "DIFFERENCE",
	Exclusion = "EXCLUSION",
	Hue = "HUE",
	Saturation = "SATURATION",
	Color = "COLOR",
	Luminosity = "LUMINOSITY",
}

/**
 * A solid color, gradient, or image texture that can be applied as fills or strokes
 * 
 * [Figma documentation](https://www.figma.com/developers/api#paint-type)
 */
export interface Paint {
	type: PaintType;
	/** Is the paint enabled? */
	visible?: boolean;
	/** Overall opacity of paint (colors within the paint can also have opacity values which would blend with this) */
	opacity?: number;
	/** Solid color of the paint */
	color?: Color;
	/** How this node blends with nodes behind it in the scene */
	blend_mode?: BlendMode;
	/** This field contains three vectors, each of which are a position in normalized object space (normalized object space is if the top left corner of the bounding box of the object is (0, 0) and the bottom right is (1,1)). The first position corresponds to the start of the gradient (value 0 for the purposes of calculating gradient stops), the second position is the end of the gradient (value 1), and the third handle position determines the width of the gradient. See image examples below: */
	gradient_handle_positions?: [Vector, Vector, Vector];
}

export enum StrokeAlign {
	/** stroke drawn inside the shape boundary */
	Inside = "INSIDE",
	/** stroke drawn outside the shape boundary */
	Outside = "OUTSIDE",
	/** stroke drawn centered along the shape boundary */
	Center = "CENTER",
}

/**
 * Animation easing curves
 * 
 * [Figma documentation](https://www.figma.com/developers/api#easingtype-type)
 */
export enum EasingType {
	/** Ease in with an animation curve similar to CSS ease-in */
	EaseIn = "EASE_IN",
	/** Ease out with an animation curve similar to CSS ease-out */
	EaseOut = "EASE_OUT",
	/** Ease in and then out with an animation curve similar to CSS ease-in-out */
	EaseInAndOut = "EASE_IN_AND_OUT",
	/** No easing, similar to CSS linear */
	Linear = "LINEAR",
	EaseInBack = "EASE_IN_BACK",
	EaseOutBack = "EASE_OUT_BACK",
	EaseInAndOutBack = "EASE_IN_AND_OUT_BACK",
	CustomBezier = "CUSTOM_BEZIER",
	Gentle = "GENTLE",
	Quick = "QUICK",
	Bouncy = "BOUNCY",
	Slow = "SLOW",
	CustomSpring = "CUSTOM_SPRING",
}

/** [Figma documentation](https://www.figma.com/developers/api#rectangle-type) */
export interface Rectangle {
	x?: number;
	y?: number;
	width?: number;
	height?: number;
}

export enum LayoutMode {
	None = "NONE",
	Horizontal = "HORIZONTAL",
	Vertical = "VERTICAL",
}

export enum StyleTypeMapKey {
	Fill = "fill",
	Fills = "fills",
	Text = "text",
	Grid = "grid",
	Effect = "effect",
	Stroke = "stroke",
	Strokes = "strokes",
}

/**
 * Metadata for character formatting
 * 
 * [Figma documentation](https://www.figma.com/developers/api#typestyle-type)
 */
export interface TypeStyle {
	/** Font family of text (standard name) */
	fontFamily: string;
	/** Numeric font weight */
	fontWeight: number;
	/** Font size in px */
	fontSize: number;
	/** Line height in px */
	lineHeightPx: number;
}

/** [Figma documentation](https://www.figma.com/developers/api#node-types) */
export interface Node {
	/** A string uniquely identifying this node within the document. */
	id: string;
	/** The name given to the node by the user in the tool. */
	name: string;
	/** Whether or not the node is visible on the canvas. */
	visible?: boolean;
	/** The type of the node */
	type: NodeType;
	/** An array of nodes that are direct children of this node */
	children?: Node[];
	/** Background color of the canvas */
	backgroundColor?: Color;
	/** An array of fill paints applied to the node */
	fills?: Paint[];
	/** An array of stroke paints applied to the node */
	strokes?: Paint[];
	/** The weight of strokes on the node */
	strokeWeight?: number;
	/** Position of stroke relative to vector outline */
	strokeAlign?: StrokeAlign;
	/** Radius of each corner of the node if a single radius is set for all corners */
	cornerRadius?: number;
	/** Array of length 4 of the radius of each corner of the node, starting in the top left and proceeding clockwise */
	rectangleCornerRadii?: [number, number, number, number];
	/** The duration of the prototyping transition on this node (in milliseconds) */
	transitionDuration?: number;
	/** The easing curve used in the prototyping transition on this node */
	transitionEasing?: EasingType;
	/** Opacity of the node */
	opacity?: number;
	/** Bounding box of the node in absolute space coordinates */
	absoluteBoundingBox?: Rectangle;
	/** The bounds of the rendered node in the file in absolute space coordinates */
	absoluteRenderBounds?: Rectangle;
	/** The distance between children of the frame. Can be negative. This property is only applicable for auto-layout frames. */
	itemSpacing?: number;
	/** Whether this layer uses auto-layout to position its children. */
	layoutMode?: LayoutMode;
	/** The padding between the left border of the frame and its children. This property is only applicable for auto-layout frames. */
	paddingLeft?: number;
	/** The padding between the right border of the frame and its children. This property is only applicable for auto-layout frames. */
	paddingRight?: number;
	/** The padding between the top border of the frame and its children. This property is only applicable for auto-layout frames. */
	paddingTop?: number;
	/** The padding between the bottom border of the frame and its children. This property is only applicable for auto-layout frames. */
	paddingBottom?: number;
	/** An array of effects attached to this node */
	effects?: Effect[];
	/** A mapping of a StyleType to style ID of styles present on this node. The style ID can be used to look up more information about the style in the top-level styles field. */
	styles?: Record<StyleTypeMapKey, string>;
	/** Text contained within a text box */
	characters?: string;
	/** Style of text including font family and weight */
	style?: TypeStyle;
}

export enum StyleType {
	Fill = "FILL",
	Text = "TEXT",
	Effect = "EFFECT",
	Grid = "GRID",
}

/** [Figma documentation](https://www.figma.com/developers/api#style-type) */
export interface Style {
	key: string;
	name: string;
	description: string;
	remote: boolean;
	styleType: StyleType;
}

export interface File {
	document: Node;
	components: Record<string, Component>;
	styles: Record<string, Style>;
	name: string;
	schemaVersion: number;
	version: string;
}

