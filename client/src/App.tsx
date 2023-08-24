import { useState, ChangeEvent, FormEvent } from "react";
import "./App.css";

export default function App() {
  const [messages, setMessages] = useState<String[]>([]);

  let socket = new WebSocket("ws://localhost:8080/ws");

  socket.onopen = function () {
    console.log("Соединение установлено");
  };

  socket.onmessage = function (event) {
    setMessages([...messages, event.data]);
  };

  socket.onclose = function (event) {
    console.log("Соединение закрыто");
  };

  socket.onerror = function (error) {
    console.log(`Ошибка: ${error}`);
  };

  const [formData, setFormData] = useState("");

  function handleChange(event: ChangeEvent<HTMLInputElement>) {
    setFormData(event.target.value);
  }

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    socket.send(formData);
    setFormData("");
  };

  return (
    <div>
      <ul>
        {messages.map((message, index) => (
          <li key={index}>{message}</li>
        ))}
      </ul>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          name="message"
          value={formData}
          onChange={handleChange}
        />
        <button>Отправить</button>
      </form>
    </div>
  );
}
