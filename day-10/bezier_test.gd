extends Node2D


var points
var p0_last = Vector2.ZERO
var p1_last = Vector2.ZERO

var c0_last = Vector2.ZERO
var c1_last = Vector2.ZERO




func _cubic_bezier_to_points(point0: Vector2, point1: Vector2, control0: Vector2, control1: Vector2):
	# https://docs.godotengine.org/en/stable/tutorials/math/beziers_and_curves.html#adding-control-points
	var curve = Curve2D.new()
	

	curve.add_point(point0, Vector2.ZERO, control0 - point0)
	curve.add_point(point1, control1 - point1, Vector2.ZERO)
	
	var curve_points = curve.tessellate()
	
#	print("ret ", curve_points)
	
	return curve_points
	

func _process(delta):
	if p0_last == $p0.position and p1_last == $p1.position  and c0_last == $c0.position  and c1_last == $c1.position:
		return
	
	p0_last = $p0.position
	p1_last = $p1.position

	c0_last = $c0.position
	c1_last = $c1.position
	
#	print("processing ", $p0.position)
	points = _cubic_bezier_to_points($p0.position, $p1.position, $c0.position, $c1.position)
#	print(points)
	
	queue_redraw()
	

	
func _draw():
	if points == null:
		print("draw wait")
		return
	
#	print(len(points))
	
	for i in len(points) - 1:
#		print("drawing ", points[i], points[i + 1])
		draw_line(points[i], points[i + 1], Color.CORAL, 5.0, true)
	
	for p in points:	
		draw_circle(p, 2.0, Color.CORNFLOWER_BLUE)
	
