import init, { EngineBridge } from './pkg/otengine.js'; 
//python -m http.server 8000
let engine;

init().then(() => {
    console.log("✅ Wasm Engine successfully loaded in background!");
    engine = new EngineBridge();
    postMessage({ type: 'ready' });
}).catch(err => {
    console.error("❌ Failed to load Wasm:", err);
});

onmessage = function(e) {
    const data = e.data;

    if (data.type === 'newgame') {
        engine.send_command("ucinewgame");
        console.log("Starting new game...");
    } 
    else if (data.type === 'search') {
        console.log(`Worker received position: ${data.history}`);
        engine.send_command(`position startpos moves ${data.history}`);
        
        const goCommand = `go wtime ${Math.max(0, data.wtime)} btime ${Math.max(0, data.btime)} winc ${data.winc} binc ${data.binc} movestogo ${data.speed}`;
        console.log(`Sending command to Rust: ${goCommand}`);
        
        const response = engine.send_command(goCommand);
        console.log(`Rust returned: ${response}`);
        
        const bestMove = response.split(' ')[1];
        postMessage({ type: 'bestmove', move: bestMove });
    }
};