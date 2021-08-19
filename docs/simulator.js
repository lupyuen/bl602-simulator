//! BL602 Simulator in WebAssembly

/// JSON Stream of Simulation Events emitted by Rust Firmware. Looks like...
///  [ { "gpio_output_set": { "pin": 11, "value": 1 } }, 
///    { "time_delay": { "ticks": 1000 } }, ... ]
let simulation_events = [];

/// Wait for emscripten to be initialised
Module.onRuntimeInitialized = function() {
  //  Load the simulator pic and render it
  const image = new Image();
  image.onload = renderSimulator;  //  Draw when image has loaded
  image.src = 'pinecone.png';      //  Image to be loaded
};

/// Render the simulator pic. Based on https://developer.mozilla.org/en-US/docs/Web/API/CanvasRenderingContext2D/drawImage
function renderSimulator() {
  //  Get the HTML canvas and context
  const canvas = document.getElementById('canvas');
  const ctx = canvas.getContext('2d');

  //  Resize the canvas
  canvas.width  = 400;
  canvas.height = 300;

  //  Draw the image to fill the canvas
  ctx.drawImage(this, 0, 0, canvas.width, canvas.height);

  //  Testing: Run the rust_script command
  if (window.location.href.indexOf("rust_script") >= 0) {
    document.getElementById("input").value = "rust_script";
    runScript();
  }
}

/// Run the command in the input box
function runScript() {
  //  Get the command, e.g. `rust_main`
  const scr = document.getElementById("input").value.trim();

  //  Get the WebAssembly Function, e.g. `Module._rust_main`
  const func = Module["_" + scr];
  if (typeof func !== "function") {
    Module.print("Unknown command: " + scr + ". Remember to add _" + scr + " to EXPORTED_FUNCTIONS in Makefile");
    return;
  }

  //  Allocate WebAssembly memory for the script
  const scr_ptr = Module.allocate(intArrayFromString(scr), ALLOC_NORMAL);

  //  Catch any errors so that we can free the allocated memory
  try {
    //  Clear the JSON Stream of Simulation Events in WebAssembly
    Module._clear_simulation_events();

    //  Execute the WebAssembly Function
    Module.print("\nExecute: " + scr + "\n");
    func();  //  TODO: Pass the command-line args

    //  Get the JSON string of Simulation Events from WebAssembly. Looks like...
    //  [ { "gpio_output_set": { "pin": 11, "value": 1 } }, 
    //    { "time_delay": { "ticks": 1000 } }, ... ]
    const json_ptr = Module._get_simulation_events();

    //  Convert the JSON string from WebAssembly to JavaScript
    const json = Module.UTF8ToString(json_ptr);

    //  Parse the JSON Stream of Simulation Events
    simulation_events = JSON.parse(json);
    Module.print("Events: " + JSON.stringify(simulation_events, null, 2) + "\n");
  } catch(err) {
    //  Catch and show any errors
    console.error(err);
  } finally {
    //  Free the WebAssembly memory allocated for the script
    Module._free(scr_ptr);
  }

  //  Start a timer to simulate the returned events
  if (simulation_events.length > 0) {
    window.setTimeout("simulateEvents()", 1);
  }

  //  More about passing strings between JavaScript and C WebAssembly:
  //  https://emscripten.org/docs/porting/connecting_cpp_and_javascript/Interacting-with-code.html
  //  https://stackoverflow.com/questions/46815205/how-to-pass-a-string-to-c-code-compiled-with-emscripten-for-webassembly
}

/// Simulate the BL602 Simulation Events recorded in simulate_events, which contains...
///  [ { "gpio_output_set": { "pin": 11, "value": 1 } }, 
///    { "time_delay": { "ticks": 1000 } }, ... ]
function simulateEvents() {
  //  Take the first event and update the queue
  if (simulation_events.length == 0) { return; }
  const event = simulation_events.shift();
  //  event looks like:
  //  { "gpio_output_set": { "pin": 11, "value": 1 } }

  //  Get the event type and parameters
  const event_type = Object.keys(event)[0];
  const args = event[event_type];

  //  Timeout in milliseconds to the next event
  let timeout = 1;

  //  Handle each event type
  switch (event_type) {

    //  Set GPIO output
    //  { "gpio_output_set": { "pin": 11, "value": 1 } }
    case "gpio_output_set": timeout += gpio_output_set(args.pin, args.value); break;

    //  Delay
    //  { "time_delay": { "ticks": 1000 } }
    case "time_delay": timeout += time_delay(args.ticks); break;

    //  Unknown event type
    default: throw new Error("Unknown event type: " + event_type);
  }

  //  Simulate the next event
  if (simulation_events.length > 0) {
    window.setTimeout("simulateEvents()", timeout);
  }
}

/// Simulate setting GPIO pin output to value 0 (Low) or 1 (High):
/// { "gpio_output_set": { "pin": 11, "value": 1 } }
function gpio_output_set(pin, value) {
  //  Get the HTML Canvas Context
  const ctx = document.getElementById('canvas').getContext('2d');

  //  Set the simulated LED colour depending on value
  switch (value) {
    //  Set GPIO to Low (LED on)
    case 0: ctx.fillStyle = '#B0B0FF'; break;  //  Blue

    //  Set GPIO to High (LED off)
    case 1: ctx.fillStyle = '#CCCCCC'; break;  //  Grey

    //  Unknown value
    default: throw new Error("Unknown gpio_output_set value: " + args.value);
  }

  //  Draw the LED colour
  ctx.fillRect(315, 116, 35, 74);

  //  Simulate next event in 0 milliseconds
  return 0;
}

/// Simulate a delay for the specified number of ticks (1 tick = 1 millisecond)
/// { "time_delay": { "ticks": 1000 } }
function time_delay(ticks) {
  //  Simulate the next event in "ticks" milliseconds
  return ticks;
}
