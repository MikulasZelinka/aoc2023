extends Node2D


var points: PackedVector2Array

var p0 = Vector2.ZERO
var p1 = Vector2.ZERO

var c0 = Vector2.ZERO
var c1 = Vector2.ZERO


var p0_last = Vector2.ZERO
var p1_last = Vector2.ZERO

var c0_last = Vector2.ZERO
var c1_last = Vector2.ZERO

var time_last: float = 0
@onready var time_slider = $TimeSlider

# if off (default), time progresses automatically (horizontal slider controls speed)
# if on, horizontal slider control time
@onready var check_button = $CheckButton

func cubic_bezier_to_points(point0: Vector2, point1: Vector2, control0: Vector2, control1: Vector2, time: float):
	# https://docs.godotengine.org/en/stable/tutorials/math/beziers_and_curves.html#adding-control-points
	var curve = Curve2D.new()
	
	curve.add_point(point0, Vector2.ZERO, control0 - point0)
	curve.add_point(point1, control1 - point1, Vector2.ZERO)
	
	
#	print(len(curve.get_baked_points()))
	
#	var curve_points = curve.tessellate()
#	var curve_points = curve.tessellate_even_length()
	var curve_points = curve.get_baked_points()
	
	var points_sliced = curve_points.slice(0, len(curve_points) * time)
	
	return points_sliced
	

func _process(delta):
	p0 = $p0.position
	p1 = $p1.position

	c0 = $c0.position
	c1 = $c1.position
	
	var time: float
	if check_button.button_pressed:
		time = time_slider.value / time_slider.max_value
	else:
		time = time_last + (delta * (time_slider.value / time_slider.max_value))
		if time > 1:
			time = 0
	
#	print(time_slider.value)
	
	
	if p0_last == p0 and p1_last == p1 and c0_last == c0 and c1_last == c1 and time_last == time:
		return
	
	p0_last = p0
	p1_last = p1

	c0_last = c0
	c1_last = c1
	
	time_last = time
	
	print(time_last)
	
#	print("processing ", $p0.position)
	points = cubic_bezier_to_points(p0, p1, c0, c1, time)
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
	
	draw_line(p0, c0, Color(Color.WHITE, 0.5), 4.0, true)
	draw_line(p1, c1, Color(Color.WHITE, 0.5), 4.0, true)
	
	
	for p in points:	
		draw_circle(p, 1.0, Color.CORNFLOWER_BLUE)
	
