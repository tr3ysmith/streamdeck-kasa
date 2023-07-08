// Global web socket
let websocket = null;
let uuid = null;
let actionInfo = null;

const sendPayloadToPlugin = (payload) => {
    if(websocket && uuid && actionInfo) {
        const json = {
            "action": actionInfo['action'],
            event: "sendToPlugin",
            context: uuid,
            payload
        }
        websocket.send(JSON.stringify(json));
    }
}

const button = document.getElementById("mybutton");
button.addEventListener("click", () => {
    button.innerHTML = "Discovering...";
    sendPayloadToPlugin({ discovery: true })
    console.log("starting discovery");
});

// Setup the websocket and handle communication
function connectElgatoStreamDeckSocket(
    inPort,
    inUuid,
    inRegisterEvent,
    inInfo,
    inActionInfo
) {
    uuid = inUuid;
    actionInfo = JSON.parse(inActionInfo);

    // Open the web socket to Stream Deck
    // Use 127.0.0.1 because Windows needs 300ms to resolve localhost
    websocket = new WebSocket(`ws://127.0.0.1:${inPort}`);

    // WebSocket is connected, send message
    websocket.onopen = () => {
        console.log("websocket connected");

        const json = {
            event: inRegisterEvent,
            uuid
        };

        websocket.send(JSON.stringify(json));
    };

    websocket.onmessage = (msg) => {
        console.log("websocket message", msg);
    };
}


// window.addEventListener('beforeunload', function (e) {
//     e.preventDefault();
//     sendPayloadToPlugin({ property_inspector: 'propertyInspectorWillDisappear' });
//     // Don't set a returnValue to the event, otherwise Chromium with throw an error.
// });