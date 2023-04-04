/*
 Generated by typeshare 1.4.0
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

/// [Figma documentation](https://www.figma.com/developers/api#vector-type)
public struct Vector: Codable {
	public let x: Double
	public let y: Double

	public init(x: Double, y: Double) {
		self.x = x
		self.y = y
	}
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

public enum StrokeAlign: String, Codable {
	/// stroke drawn inside the shape boundary
	case inside = "INSIDE"
	/// stroke drawn outside the shape boundary
	case outside = "OUTSIDE"
	/// stroke drawn centered along the shape boundary
	case center = "CENTER"
}

public enum PaintType: String, Codable {
	case solid = "SOLID"
	case gradientLinear = "GRADIENT_LINEAR"
	case gradientRadial = "GRADIENT_RADIAL"
	case gradientAngular = "GRADIENT_ANGULAR"
	case gradientDiamond = "GRADIENT_DIAMOND"
	case image = "IMAGE"
}
