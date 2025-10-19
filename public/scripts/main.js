let roomId = null;
let user = null;
let eventSource = null;

let API_BASE;

if (
  window.location.hostname !== "127.0.0.1" &&
  window.location.hostname !== "localhost"
) {
  API_BASE = window.location.origin;
} else {
  API_BASE = "http://127.0.0.1:8000";
}

function generateUUID() {
  return "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(/[xy]/g, function (c) {
    const r = (Math.random() * 16) | 0,
      v = c == "x" ? r : (r & 0x3) | 0x8;
    return v.toString(16);
  });
}

function updateRoomDisplay(newRoomId) {
  if (newRoomId) {
    roomId = newRoomId;
    document.querySelector("#current-room-id").innerText = `Room: ${roomId}`;
    document.querySelector(".room").innerHTML =
      `<h2>Connected to Room ${roomId}</h2>`;
  } else {
    document.querySelector("#current-room-id").innerText = "Room: N/A";
    document.querySelector(".room").innerHTML = "";
  }
}

const connectToStream = function (roomId) {
  if (eventSource) {
    eventSource.close();
    console.log("Previous SSE connection closed.");
  }

  updateRoomDisplay(roomId);

  eventSource = new EventSource(`${API_BASE}/rooms/${roomId}/stream`);

  eventSource.onopen = function () {
    console.log("SSE connection opened and listening for messages.");
  };

  eventSource.onmessage = function (event) {
    try {
      const message = JSON.parse(event.data);
      displayMessage(message);
    } catch (e) {
      console.error("Error parsing JSON message:", e, event.data);
      displayMessage({ sender: { username: "SERVER" }, content: event.data });
    }
  };

  eventSource.onerror = function (err) {
    console.error("SSE connection error. Trying to reconnect...", err);
  };
};

const displayMessage = function (message) {
  const roomDiv = document.querySelector(".room");
  const el = document.createElement("div");
  el.innerText = `[${message.sender.username}]: ${message.content}`;
  roomDiv.appendChild(el);
  roomDiv.scrollTop = roomDiv.scrollHeight;
};

const initializeUser = function () {
  if (user) return true;

  const usernameInput = document.querySelector('input[name="username"]').value;
  if (!usernameInput) {
    alert("Please enter a username.");
    return false;
  }

  user = {
    id: generateUUID(),
    username: usernameInput,
  };
  console.log("User initialized:", user);
  return true;
};

const create_room = async function () {
  if (!initializeUser()) return;

  try {
    const response = await fetch(`${API_BASE}/rooms`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ user }),
    });

    if (!response.ok)
      throw new Error(`Error creating room: ${response.statusText}`);

    const data = await response.json();

    console.log("Room created:", data);
    connectToStream(data);
  } catch (error) {
    alert(`Failed to create room: ${error.message}`);
  }
};

const join_room_by_id = async function () {
  if (!initializeUser()) return;

  const inputRoomId = document.querySelector(
    'input[name="room_id_input"]',
  ).value;
  if (!inputRoomId) {
    return alert("Please enter a room ID to join.");
  }

  try {
    const response = await fetch(`${API_BASE}/rooms/${inputRoomId}/join`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ user }),
    });

    if (!response.ok) {
      const error = await response.json();
      throw new Error(`Error joining room: ${JSON.stringify(error)}`);
    }

    console.log(`Joined room: ${inputRoomId}`);
    connectToStream(inputRoomId);
  } catch (error) {
    alert(`Failed to join room: ${error.message}`);
  }
};

const send_message = async function () {
  if (!roomId) return alert("Please create or join a room first.");
  if (!user) return alert("User not initialized.");

  const contentInput = document.querySelector('input[name="content"]');
  const content = contentInput.value.trim();

  if (!content) return;

  const message = {
    id: generateUUID(),
    sender: user,
    content: content,
  };

  try {
    const response = await fetch(`${API_BASE}/rooms/${roomId}`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ message }),
    });

    if (!response.ok)
      throw new Error(`Error sending message: ${response.statusText}`);

    contentInput.value = "";
  } catch (error) {
    alert(`Failed to send message: ${error.message}`);
  }
};
