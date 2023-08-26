import { Middleware } from "redux";
import {
  startConnecting,
  connectionEstablished,
  receiveMessage,
  submitMessage,
} from "../slices/chatSlice";

const chatMiddleware: Middleware = (store) => {
  let socket: WebSocket;

  return (next) => (action) => {
    const isConnectionEstablished = socket && store.getState().chat.isConnected;

    if (startConnecting.match(action)) {
      socket = new WebSocket("ws://localhost:8080/ws");

      socket.onopen = function () {
        console.log("Соединение установлено"); // !
        store.dispatch(connectionEstablished());
      };

      // receiving and sending messages !

      socket.onmessage = function (event) {
        store.dispatch(receiveMessage(JSON.parse(event.data)));
      };
    }

    if (submitMessage.match(action) && isConnectionEstablished) {
      socket.send(JSON.stringify(action.payload));
    }

    next(action);
  };
};

export default chatMiddleware;
