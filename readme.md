# Rudotai 
I create this library mostly with AI code to create general object generation from number and manipulation of object shape.

## What Dot Defined Here ?
In mathematics, a dot typically refers to a point in a 2-dimensional or 3-dimensional space. A dot can be represented by its coordinates, which give its location in the space. In 2-dimensional space, a dot is usually represented by two numbers, while in 3-dimensional space, it is represented by three numbers.

Dots are commonly used in various fields such as geometry, physics, and computer science. In computer science, dots are often used as a way to represent objects in a 2-dimensional or 3-dimensional space. They can be used to represent images, 3D models, and other types of visual data.

Dot also can refer into the last point of needl tip , a square , or a cube depends on zoom levels of viewport and camera, its also can refer into a pixel in bits.

## How Dot Terms Used Here

Dot - 1 Dimensional Dot is called Dot and only contains one position variable (mostly useless) , but good for positioning the initial object or as starting point of creation or empty struct , architecture or object , to start generation.

Dot2d - 2D context Dimensional Dot use x,y cartesian system this can be rendered inside view port already

Dot3d - 3D context Dimensional Dot use x,y,z like Vertices system.

## How do we create something from dot ?

The Answer Lies from Geometry , Cartesian Coordinate , ViewPort (Camera),Subject And Object,Vertices. Then a Random Data classified or identified likely as input or refrence.

## Design Principal 

Point can be threated as position.
Dot must be threated as object Of view.
DotCam is a dot with point of view or eyes available (Dot With Camera).

#### Charateristic Of Dot Inherited Object

```
- It must can be turned into Object represent by number, floating point , or string only
- Inner Boundary and Outside Boundary Aware
- Collision Aware with other Object (For Events)
- View Port Aware To Minimize Memory Use (For Freeze state or not)
- Input of dot ui must be 2 floating point number in 2d space
- Input of dot ui must be 3 floating point number in 3d space
- It must can calculate its internal or external bounds using intersection
- It must can be drawed on 2D context and 3D context without care about the renderer
- It must can calculate collision using collision math and innerbound and outerbound data
- It must can be styled by providing a colors (not a theme)
- It must represent the shape its named in the struct (need to gather geometry study from 2d context data model and 3D vertex data model) for comparison on training
- The Scale of Dot object depends on external environment by dividing screensize with ratio of object acquired from dividing it from View Port or Camera (Point Of View)
- Didn't know concept pipeline so make it pipeline free as its just containt raw data
- Use Scale Factor to make the Dot Bigger or Smaller.
- Width , Height , Length , Size, Middle,Intersect on use in 1-2d Context
- Area , Volume , Surface , Stack, Collision on use in 3d context 
- morph it from one value variable dot, if there is no value on other variable as input calc it with comparison from other shape data model.
```

Basic Object Example

```

// Dot is a Simple Point But Threated as Object
#[derive(Debug, Copy, Clone)]
pub struct Dot {
    pub x: f64,
    pub y: f64,
}

// Rectangle must be created from Dot x and y to unleash power of inheritance and generation
#[derive(Debug, Copy, Clone)]
struct Rectangle {
    bottom_left: Dot,
    top_right: Dot,
}

// Circle Also must be created from Dot x and y to unleash power of inheritance and generation
#[derive(Debug, Copy, Clone)]
pub struct Circle {
    pub center: Dot,
    pub radius: f64,
}

```

lets make Rudotai that aware every draw is started from dot whatever the dot , that can morph like this.

dot->dot2d->line->triangle:square:circle->dot3d->sphere->dot

after morph is done its only matter of stack to create environment of 3D spaces.

this readme is still on going as i didn't know how usefull will be this lib.

### for what of use the generation data?

modelling best path and line to generate most efficient method on object creation and animation from just a number to train in AI model based on vote the object is correct or not.