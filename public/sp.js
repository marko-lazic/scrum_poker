setTimeout(function(){ 
    window.ipc.ws.addEventListener("close", (event) => {
        setTimeout(function() {
            window.location.reload(true);
        }, 1000);
    });
}, 3000);  
