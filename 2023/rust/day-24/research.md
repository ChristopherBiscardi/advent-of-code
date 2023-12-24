

The point of contact must be

```
our_hailstone_starting_position + t * our_hailstone_direction
```

for every other hailstone

so solving for that gives up

# id: "e1"
```
our_hailstone_starting_position + t[n] * our_hailstone_direction == hail_n.starting_position + t[n] * hail_n.direction
```

which is 6 variables.

```
our_hailstone_starting_position + t[n] * (our_hailstone_direction - hail_n.direction) - hail_n.starting_position == 0
```

unknowns:
* our_hailstone.starting_position[x,y,z]
* our_hailstone.direction[x,y,z]
* t?

our_hailstone_starting_position + t[0] * our_hailstone_direction == hail_0.starting_position + t[0] * hail_0.direction
our_hailstone_starting_position + t[1] * our_hailstone_direction == hail_1.starting_position + t[1] * hail_1.direction
our_hailstone_starting_position + t[2] * our_hailstone_direction == hail_2.starting_position + t[2] * hail_2.direction


t[0] * our_hailstone_direction.x - (t[0] * hail_0.direction.x) == hail_0.starting_position.x - our_hailstone_starting_position.x
t[0] * (our_hailstone_direction.x - hail_0.direction.x) == hail_0.starting_position.x - our_hailstone_starting_position.x
0 == (hail_0.starting_position.x - our_hailstone_starting_position.x) cross (our_hailstone_direction.x - hail_0.direction.x)

0 == (hail_0.starting_position.x - our_hailstone_starting_position.x) cross (our_hailstone_direction.x - hail_0.direction.x)


