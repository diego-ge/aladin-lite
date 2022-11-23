import init, * as module from 'alv3';

onmessage = async function ({ data }) {
    console.log("memory location of wasm", data.mem)
    //await init();
    await init();
    const num = new Uint32Array(Array.from(Array(10000).keys()));
    console.log(num)
    let result = module.sum(num);
    console.log("HEAVY WORK", result);

    postMessage("done");
};