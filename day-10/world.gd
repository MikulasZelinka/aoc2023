extends Node2D


const valid_neighbours = {
	# UP
	[-1, 0]: {"7": null, "|": null, "F": null},
	# DOWN
	[1, 0]: {"J": null, "|": null, "L": null},
	# LEFT
	[0, -1]: {"L": null, "-": null, "F": null},
	# RIGHT
	[0, 1]: {"J": null, "-": null, "7": null},
	
}

func _ready():
	assert(part1("res://example1.txt") == 4)
	assert(part1("res://example2.txt") == 8)
	print(part1("res://input.txt"))
	

func read_file(file_path):
	# https://docs.godotengine.org/en/stable/classes/class_fileaccess.html
	var file = FileAccess.open(file_path, FileAccess.READ)
	var content = file.get_as_text()
	return content


func map_str_to_dict(map: String):
	var lines = map.split("\n", false)
#	print(lines)
	
	var starting_position = [0, 0]
	
	var coords_to_char = {}
	
	var row = 1
	for line in lines:
		var col = 1
		for char in line:
			coords_to_char[[row, col]] = char
			
			if char == "S":
				starting_position = [row, col]
			
			col += 1
		row += 1
	
	assert(starting_position != [0, 0])
	return [coords_to_char, starting_position]
			
		
	

#func can_move_from_to_target(from: Array, to: Array, map: Dictionary):
#	return valid_neighbours[from - to].has()

func part1(file_path):
	var map_str = read_file(file_path)
	
	var coords_to_char_starting_position = map_str_to_dict(map_str)
	var coords_to_char = coords_to_char_starting_position[0]
	var starting_position = coords_to_char_starting_position[1]
	
	var last = starting_position
	var last_char = "S"
	
	var valid_starts = []
	
	for dir in valid_neighbours.keys():
		var next = [last[0] + dir[0], last[1] + dir[1]]
		var next_char = coords_to_char.get(next)
		
		if next_char == null:
			continue
	
		if valid_neighbours.get(dir).has(next_char):
			valid_starts.append(next)
			
	
#	print(coords_to_char)
#	print(starting_position)
#
#	print(valid_starts)
	assert(len(valid_starts) == 2)
	
	last = valid_starts[0]
	last_char = coords_to_char[last]
	var end = valid_starts[1]
	
	var pipe_path = {starting_position: null, last: null}
	
	while last != end:
		for dir in valid_neighbours.keys():
			var next = [last[0] + dir[0], last[1] + dir[1]]
			var next_char = coords_to_char.get(next)
			
			if pipe_path.has(next) or next_char == null:
				continue
		
			if valid_neighbours.get(dir).has(next_char) and valid_neighbours.get([-dir[0], -dir[1]]).has(last_char):
				last = next
				last_char = coords_to_char[last]
#				print(next, next_char)
				pipe_path[last] = null
#				print(pipe_path)
	
#	print(len(pipe_path), pipe_path)
	
	
	return len(pipe_path) / 2

