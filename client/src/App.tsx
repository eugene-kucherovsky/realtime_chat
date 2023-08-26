import { useState, useEffect, ChangeEvent, FormEvent } from "react";
import { useAppDispatch, useAppSelector } from "./redux/store";
import { startConnecting, submitMessage } from "./redux/slices/chatSlice";
import "./App.css";

export type ChatMessage = {
  id: string;
  messageText: string;
};

export default function App() {
  const dispatch = useAppDispatch();
  const messages = useAppSelector((state) => state.chat.messages);

  useEffect(() => {
    dispatch(startConnecting());
  }, []);

  const [formData, setFormData] = useState({
    id: "",
    messageText: "",
  });

  function handleChange(event: ChangeEvent<HTMLInputElement>) {
    const { name, value } = event.target;
    setFormData((prevState) => {
      return { ...prevState, [name]: value };
    });
  }

  const handleSubmit = (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();

    if (formData.messageText === "") {
      return;
    } else {
      dispatch(submitMessage(formData));
      setFormData({
        id: "",
        messageText: "",
      });
    }
  };

  return (
    <div>
      <ul>
        {messages.map((message, index) => (
          <li key={index}>{message.messageText}</li>
        ))}
      </ul>
      <form onSubmit={handleSubmit}>
        <input
          type="text"
          name="messageText"
          value={formData.messageText}
          onChange={handleChange}
        />
        <button>Отправить</button>
      </form>
    </div>
  );
}
