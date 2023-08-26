import { createSlice, PayloadAction } from "@reduxjs/toolkit";

export type ChatMessage = {
  id: string;
  messageText: string;
};

export type ChatState = {
  messages: ChatMessage[];
  isEstablishingConnection: boolean;
  isConnected: boolean;
};

const initialState: ChatState = {
  messages: [],
  isEstablishingConnection: false,
  isConnected: false,
};

const chatSlice = createSlice({
  name: "chat",
  initialState,
  reducers: {
    startConnecting: (state) => {
      state.isEstablishingConnection = true;
    },
    connectionEstablished: (state) => {
      state.isConnected = true;
      state.isEstablishingConnection = true;
    },
    receiveMessage: (state, action: PayloadAction<ChatMessage>) => {
      state.messages.push(action.payload);
    },
    submitMessage: (_state, _action: PayloadAction<ChatMessage>) => {
      return;
    },
  },
});

export const {
  startConnecting,
  connectionEstablished,
  receiveMessage,
  submitMessage,
} = chatSlice.actions;

export default chatSlice.reducer;
