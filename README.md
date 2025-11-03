# SpireTween

An addon that provides a fast and ergonomic tweening library for Godot 4.x, inspired on the design of the
popular Unity library [DoTween](https://dotween.demigiant.com/).

These are the priorities of SpireTween:

- Ergonomics without compromising performance (see the section [Benchmarks](#benchmarks)).
- Type safety, taking advantage of GDScript's type system to catch errors at compile time rather than runtime.
- "No surprises": Spire's public API is thoroughly (and proudly!) documented,
  with method descriptions mentioning possible side effects and quirks. I also wrote plenty of integration tests to make sure the behavior
  matches what's written in the docs (see folder [tests](spire_tween_gdscript/examples/tests)).
  Here's an example: The method `play` that all tweens have (screenshot taken from inside Godot's editor):
  ![docs_method_play.png](readme_images/docs_method_play.png)
- At minimum, feature parity with Godot's built-in tweening system (after all, why would you want a downgrade?).
  SpireTween also provides more features, such as: speed-based tweens, more loop modes, etc.

# Installation

SpireTween is an addon like any other, just copy the addon folder into "res://addons/".

(That's it)

# Configuration

Currently, SpireTween supports a single global configuration option, which lets you define what is the default "Ease" type when creating tweens.
You can set this option by calling `SpireGlobalSettings.set_default_ease`:

```gdscript
func _ready():
    # Set the default easing to "In-Out Sine", which is Godot's built-in Tween's default easing.
    SpireGlobalSettings.set_default_ease(Spire.EASE_IN_OUT_SINE)
```

If not set, the default is `Spire.EASE_LINEAR`.

# Cheat Sheets: Builtin Tween -> SpireTween

## Cheat Sheet: Type analogies

*(From here on, the names in the table bellow may be used interchangeably.)*

| Godot Built-in  | SpireTween                                          |
|-----------------|-----------------------------------------------------|
| Tween           | SpireSequence                                       |
| PropertyTweener | SpireProperty{Type}                                 |
| MethodTweener   | SpireMethod{Type}                                   |
| CallbackTweener | SpireDelayedCall                                    |
| IntervalTweener | No type, use `SpireSequence.append_interval(float)` |
| SubtweenTweener | SpireSequence[note1](#note-subtweeners)             | 

<a name="note-subtweeners"></a>
Note 1: A `SpireSequence` is itself a `SpireTween`, so you can append sequences into others,
recursively.

**Note:** In the tables below, the columns "Godot Class(es)" and "Spire Class(es)" indicate which classes support the feature.

## Cheat Sheet: Properties

Cheat sheet for reading/writing properties/configuration that are common to both systems:

| Property                                                                |                    Godot Class(es)                    |        Godot Read        |                     Godot Write                      |          Spire Class(es)           |        Spire Read        |                          Spire Write                          |
|-------------------------------------------------------------------------|:-----------------------------------------------------:|:------------------------:|:----------------------------------------------------:|:----------------------------------:|:------------------------:|:-------------------------------------------------------------:|
| Loops (total) ([note1](#note-loop-modes))                               |                         Tween                         |            ❌             |                    set_loops(int)                    |                All                 |       get_loops()        |                set_loops(int, Spire.LoopMode)                 |
| Loops Left                                                              |                         Tween                         |     get_loops_left()     |                          ❌                           |                All                 |     get_loops_left()     |                               ❌                               |
| Loops Completed                                                         |                           ❌                           |            ❌             |                          ❌                           |                All                 |   get_loops_finished()   |                               ❌                               |
| Loop Mode                                                               |                           ❌                           |            ❌             |                          ❌                           |                All                 |     get_loop_mode()      |                 set_loop_mode(Spire.LoopMode)                 |
| Run                                                                     |                         Tween                         |       is_running()       |                        play()                        |                All                 |       is_playing()       |                            play()                             |
| Pause                                                                   |                         Tween                         |      !is_running()       |                       pause()                        |                All                 |       is_paused()        |                            pause()                            |
| Stop                                                                    |                         Tween                         |            ❌             |                        stop()                        |                All                 |       is_stopped()       |                            stop()                             |
| Validity ([note2](#note-is-valid))<br/>(is it "ticked" on each update?) |                         Tween                         |        is_valid()        |                        kill()                        |                All                 |     is_registered()      |                  register()<br/>unregister()                  |
| Delay                                                                   |           PropertyTweener<br/>MethodTweener           |            ❌             |                   set_delay(float)                   |                All                 |       get_delay()        |                       set_delay(float)                        |
| Elapsed Time (total)                                                    |                         Tween                         | get_total_elapsed_time() |                          ❌                           |                All                 | get_total_elapsed_time() |                               ❌                               |
| Animation position (current loop's elapsed time, after any delays)      |                         Tween                         |            ❌             |                          ❌                           |                All                 | get_animation_position() |                               ❌                               |
| Ignore Time Scale ([note3](#note-ignore-time-scale))                    |                         Tween                         |            ❌             |             set_ignore_time_scale(bool)              |                All                 | get_ignore_time_scale()  |                  set_ignore_time_scale(bool)                  |
| Speed Scale                                                             |                         Tween                         |            ❌             |                set_speed_scale(float)                |                All                 |    get_speed_scale()     |                    set_speed_scale(float)                     |
| Pause Mode                                                              |                         Tween                         |            ❌             |            set_pause_mode(TweenPauseMode)            |                All                 |     get_pause_mode()     |                set_pause_mode(Spire.PauseMode)                |                                                               
| Process Mode                                                            |                         Tween                         |            ❌             |          set_process_mode(TweenProcessMode)          |                All                 |    get_process_mode()    |              set_process_mode(Spire.ProcessMode)              |                                                               
| Lifetime Bound ([note4](#note-bind-node)                                |                         Tween                         |            ❌             |                   bind_node(Node)                    |                All                 |    get_bound_nodes()     | bind_node(Node)<br/>unbind_node(Node)<br/>clear_bound_nodes() |
| Easing (enum)                                                           |                    PropertyTweener                    |            ❌             |    set_ease(EaseType) + set_trans(TransitionType)    |   SpireProperty<br/>SpireMethod    |        get_ease()        |                     set_ease(Spire.Ease)                      |
| Easing (func)                                                           |                    PropertyTweener                    |            ❌             |          set_custom_interpolator(Callable)           |   SpireProperty<br/>SpireMethod    |        get_ease()        |                    set_ease_func(Callable)                    |
| Easing (curve)                                                          |                    PropertyTweener                    |            ❌             | set_custom_interpolator(curve_variable.sample_baked) |   SpireProperty<br/>SpireMethod    |        get_ease()        |                     set_ease_curve(Curve)                     |
| Relative Mode                                                           |                    PropertyTweener                    |            ❌             |                    as_relative()                     |           SpireProperty            |      is_relative()       |                         as_relative()                         |
| Speed-Based Mode                                                        |                           ❌                           |            ❌             |                          ❌                           |           SpireProperty            |     is_speed_based()     |                       as_speed_based()                        |
| Start Value (property)                                                  |                    PropertyTweener                    |            ❌             |                    from(Variant)                     |           SpireProperty            |            ❌             |                         from({Type})                          |
| Start Value (property) (as current)                                     |                    PropertyTweener                    |            ❌             |                    from_current()                    |           SpireProperty            |            ❌             |         Not necessary, this is the default behavior.          |
| Start Value (method)                                                    |                     MethodTweener                     |            ❌             |             Set when creating the tween.             |            SpireMethod             |    get_start_value()     |                 Set when creating the tween.                  |
| Final Value                                                             |           PropertyTweener<br/>MethodTweener           |            ❌             |             Set when creating the tween.             |   SpireProperty<br/>SpireMethod    |    get_final_value()     |                 Set when creating the tween.                  |
| Tweened Property's path                                                 |                    PropertyTweener                    |            ❌             |             Set when creating the tween.             |           SpireProperty            |   get_property_path()    |                 Set when creating the tween.                  |
| Object that owns the property/method being tweened                      |                    PropertyTweener                    |            ❌             |             Set when creating the tween.             |           SpireProperty            |       get_owner()        |                 Set when creating the tween.                  |
| Duration                                                                | PropertyTweener<br/>MethodTweener<br/>IntervalTweener |            ❌             |             Set when creating the tween.             | SpireProperty<br/>SpireMethod<br/> |      get_duration()      |                 Set when creating the tween.                  |
| Callable being tweened                                                  |                     MethodTweener                     |            ❌             |             Set when creating the tween.             |  SpireMethod<br/>SpireDelayedCall  |      get_callable()      |                 Set when creating the tween.                  |

Although both systems share many properties, Godot's built-in `Tween` does not expose getters for most of them, while SpireTween exposes getters to almost all.
This isn't a big deal, the maintainers of Godot probably decided to not expose those getters since most use cases wouldn't need to read those properties after creating the tween.
That's a reasonable design decision, though I have a different philosophy: I can't predict all possible use cases, so I prefer to expose more information rather than less, specially when
doing so is trivial/harmless.

<a name="note-loop-modes"></a>

**Note 1**: Spire supports 3 different loop modes: Restart (the default), Yoyo, and Incremental. See the enum's documentation for more details.

<a name="note-is-valid"></a>

**Note 2**: In Godot's builtin `Tween`, `kill()` is a permanent operation.
In Spire, `unregister()` simply removes a tween from the internal update list, it can be re-registered again later with `register()`.
Since Spire's tweens are reference-counted, there's no need to manually free them (which is what `kill()` is supposed to do).

<a name="note-ignore-time-scale"></a>

**Note 3**: In Godot's builtin `Tween`, this is only available for the `Tween` class (equivalent to Spire's `SpireSequence`), and it affects all
child "tweeners". In Spire, the behavior is the same for `SpireSequence`, but it is also available to individual tweeners.

<a name="note-bind-node"></a>
**Note 4**: Godot only supports binding to a single node, while Spire supports binding to multiple nodes. Also, the semantics of binding are slightly different on each
system, see their documentation for more details.

## Cheat Sheet: Methods

| Feature                            | Godot Class(es) |                                                  Godot "how to"                                                   | Spire Class(es) |                                          Spire "how to"                                          |
|------------------------------------|:---------------:|:-----------------------------------------------------------------------------------------------------------------:|:---------------:|:------------------------------------------------------------------------------------------------:|
| Manual Stepping                    |      Tween      |                                                custom_step(float)                                                 |       All       |                                        custom_step(float)                                        |
| Instant completion                 |      Tween      |                                           custom_step(very_big_number)                                            |       All       |                                         force_complete()                                         |
| Kill                               |      Tween      |                                                      kill()                                                       |       All       |                                           unregister()                                           |
| Resuscitate ("Unkill")             |        ❌        |                                                         ❌                                                         |       All       |                                            register()                                            |
| Sequencing - Series                |      Tween      | Tweener builder methods (`Tween.tween_*()`) do this automatically.<br/>More complex cases require using `chain()` |  SpireSequence  | append(AnyTweenType)<br/>append_call(Callable)<br/>append_interval(float)<br/>append_many(Array) |
| Sequencing - Parallel              |      Tween      |            Call `set_parallel(true)`, then use one of the builder methods (`Tween.tween_*()`) as usual            |  SpireSequence  |       join(AnyTweenType)<br/>join_call(Callable)<br/>join( interval)<br/>join_many(Array)        |
| Sequencing - Insertion             |        ❌        |                                                         ❌                                                         |  SpireSequence  |                   insert(float, AnyTweenType)<br/>insert_call(float, Callable)                   |
| Sequencing - Remove child tween    |        ❌        |                                                         ❌                                                         |  SpireSequence  |                          remove(AnyTweenType)<br/>remove_call(Callable)                          |
| Sequencing - Default children ease |      Tween      |                                  set_ease(EaseType) + set_trans(TransitionType)                                   |  SpireSequence  |                              set_default_children_ease(Spire.Ease)                               |

## Cheat Sheet: Signals

| Emission Condition                    | Godot Class(es) |          Godot Signal          | Spire Class(es) |               Spire Signal               |
|---------------------------------------|:---------------:|:------------------------------:|:---------------:|:----------------------------------------:|
| Completed (Entire tween)              |      Tween      |           finished()           |       All       |                finished()                |
| Completed (Single loop)               |      Tween      | loop_finished(loop_count: int) |       All       | loop_finished()[note1](#note-loop-count) |
| Sequencing - Parallel Group completed |      Tween      |       step_finished(int)       | ~SpireSequence  |  Use the children's "finished" signals.  |

<a name="note-loop-count"></a>
**Note 1**: `loop_count` isn't provided with the signal but can be retrieved with `get_loops_finished()`.

## Cheat Sheet: Creating a Tween(er)

| Tween type                                     | Godot "how to"                                                                                  | Spire "how to"                                                                                                                                                                             |
|------------------------------------------------|-------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| Property by "inlined" name                     | ❌                                                                                               | Do{Class}.{property_name}(object: {Class}, to: {Type}, duration: float)                                                                                                                    |
| Property by path (typed)                       | ❌                                                                                               | Spire.do_property_{type}(object: Object, property: NodePath, to: {Type}, duration: float)                                                                                                  |
| Property by path (untyped)                     | create_tween().tween_property(object: Object, property: NodePath, to: Variant, duration: float) | Spire.do_property(object: Object, property: NodePath, to: Variant, duration: float)                                                                                                        |
| Property by path (custom/non-primitive type)   | create_tween().tween_property(object: Object, property: NodePath, to: Variant, duration: float) | Spire.do_property_custom(object: Object, property: NodePath, to: Variant, duration: float, distance_func: Callable, lerp_func: Callable, add_relative_func: Callable, step_func: Callable) |
| Method by callable (typed)                     | ❌                                                                                               | Spire.do_call_{type}(callable: Callable, from: {Type}, to: {Type}, duration: float)                                                                                                        |
| Method by callable (untyped)                   | create_tween().tween_method(method: Callable, from: Variant, to: Variant, duration: float)      | Spire.do_call(callable: Callable, from: Variant, to: Variant, duration: float)                                                                                                             |
| Method by callable (custom/non-primitive type) | create_tween().tween_method(method: Callable, from: Variant, to: Variant, duration: float)      | Spire.do_call_custom(callable: Callable, from: Variant, to: Variant, duration: float, lerp_func: Callable)                                                                                 |
| Delayed Callback                               | create_tween().tween_callback(callback: Callable).set_delay(delay: float)                       | Spire.do_delayed_call(callable: Callable, delay: float)                                                                                                                                    |

# Examples

*For "live" example scenes that you can play, see folder [examples](spire_tween_gdscript/examples).*

## Tweening Properties

### Common properties of built-in classes

Godot's built-in system provides a single method to create all property tweeners: [Tween.tween_property]
(https://docs.godotengine.org/en/stable/classes/class_tween.html#class-tween-method-tween-property). Because of that,
it cannot provide type safety and the checks must be performed at runtime.

Spire takes two approaches, the first is that it provides many static functions that will tween a specific property of a
specific class, these are presented in the format:

```
Do{NodeClass}.{property_name}(node: {NodeClass}, to: {Type}, duration: float) -> SpireProperty{Type}
   ^^^^^^^^^                                          ^^^^^^^^^^^^
# Node2D, CanvasItem, etc.                          float, Vector2, etc.
```

Examples: `DoNode2D.global_position`, `DoCanvasItem.modulate_a`, `DoControl.rotation`.

Since we know the class and property at compile time, the GDScript compiler will yell if you accidentally try to pass bad arguments.

The second approach, which is meant for cases where you want to tween a custom property, is to provide an API similar to
Godot's built-in, except that the API is split into multiple methods based on the type being tweened.

These static functions are available in the `Spire` class:

```
do_property(owner: Object, property_path: NodePath, to: Variant, duration: float) -> SpireProperty
do_property_int(owner: Object, property_path: NodePath, to: int, duration: float) -> SpirePropertyInt
do_property_float(owner: Object, property_path: NodePath, to: float, duration: float) -> SpirePropertyFloat
do_property_vec2(owner: Object, property_path: NodePath, to: Vector2, duration: float) -> SpirePropertyVector2
do_property_color(owner: Object, property_path: NodePath, to: Color, duration: float) -> SpirePropertyColor
# ... and so on for Vector2i, Vector3, Vector3i, String and Variant.
```

Example: Tweening the property `global_position` of a Node2D.

```gdscript
var node: Node2D # or any other type that inherits it
var destination: Vector2
var duration: float

# Godot
node.create_tween().tween_property(node, ^"global_position", destination, duration)

# Spire
# Signature: func(node: Node2D, to: Vector2, duration: float) -> SpirePropertyVector2 
DoNode2D.global_position(node, destination, duration)
# For this specific property that's so commonly used, Spire also provides a shorthand:
DoNode2D.move(node, destination, duration)
```

Example: Tweening the component `x` of the property `global_position` of a Node2D.

```gdscript
extends Node2D

var destination_x: float = 500.0
var duration: float = 4.0

func _ready():
    # Godot
    node.create_tween().tween_property(node, ^"global_position:x", destination_x, duration)
    
    # Spire
    # Signature: func(node: Node2D, to: float, duration: float) -> SpirePropertyFloat
    DoNode2D.global_position_x(node, destination_x, duration)
    # For this specific property that's so commonly used, Spire also provides a shorthand:
    DoNode2D.move_x(node, destination_x, duration)
```

### Custom/uncommon properties

Example: Tweening a custom property of type `int`.

```gdscript
extends Node

var _property_backing_field: int = 0
var property: int:
    get: return _property_backing_field
    set(value: int): _property_backing_field = value

var to: int = 30
var duration: float = 4.0

func _ready():
    # Godot
    self.create_tween().tween_property(self, ^"property", to, duration)
    # Spire
    # Signature: func(obj: Object, property_path: NodePath, to: int, duration: float) -> SpirePropertyInt
    Spire.do_property_int(self, ^"property", to, duration)
```

## Tweening Methods

Example: Tweening a method that sets an `Vector2i` value.

```gdscript
extends Node

var from: Vector2i = Vector2i.ZERO
var to: Vector2i = Vector2i(100, 200)
var duration: float = 4.0

func _ready():
    # Godot
    self.create_tween().tween_method(print_value, from, to, duration)
    # Spire
    # Signature: func(callable: Callable, from: Vector2i, to: Vector2i, duration: float) -> SpireMethodVector2i
    Spire.do_call_vec2i(print_value, from, to, duration)

func print_value(value: Vector2i) -> void:
    print("Value set to: " + str(value))
```

## Tweening in Series/Parallel

*For more details on how sequences work, see the class documentation of `SpireSequence`.*

Example: Moving a sprite along the sides of a rectangle, as well as making it flash red every time it reaches a corner.

```gdscript
extends Sprite

var vertices := [
    Vector2(100, 100),
    Vector2(500, 100),
    Vector2(500, 400),
    Vector2(100, 400),
]

var speed := 200.0 # pixels per second
var flash_duration := 0.5

func _ready():
    var seq := Spire.sequence().set_loops(-1) # Make the sprite run around the rectangle indefinitely.
    
    for vert: Vector2 in vertices:
        # `append` creates a new "step" in the sequence with a single tween inside.
        seq.append(DoNode2D.move(self, vert, speed).as_speed_based())
        
        # `join` adds another tween to the current step of the sequence.
        # In this case, it makes the tween bellow run(flash) at the same time as the tween above(movement).
        seq.join(
            DoCanvasItem.modulate(self, Color(1, 0, 0, 1), flash_duration)
                .set_loops(2, Spire.LOOP_MODE_YOYO) # yoyo loop will make it go red then back to the original color.
        )

```

# Benchmarks