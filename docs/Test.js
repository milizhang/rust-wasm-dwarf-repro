let wasm;

export function testFn()
{
    var ret = wasm.testFn();
    return ret;
}

async function load()
{
	wasm = (await WebAssembly.instantiateStreaming(fetch("RustWasmTest.wasm"), {})).instance.exports;
}

load().then( () =>
{
	document.getElementById("btn").onclick = () =>
	{
		document.getElementById("tf").value = testFn();
	}
});

