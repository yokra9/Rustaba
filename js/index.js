import("../pkg/index.js").catch(console.error);

async function t() {
    const wasm = await import("../pkg/");
    return wasm.parse(`
      
   
    `)
}

t().then(json => {
    var a = JSON.parse(json)
    console.log(a)
})