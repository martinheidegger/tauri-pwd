import { useState, useEffect } from 'react'
import reactLogo from './assets/react.svg'
import viteLogo from '/vite.svg'
import './App.css'
import { invoke } from "@tauri-apps/api/tauri"

function App() {
  const [count, setCount] = useState(0)
  const [password, setPassword] = useState("")
  const [storedHash, setStoredHash] = useState("")

  useEffect(() => {
    loadStoredHash()
  }, [])

  async function savePassword() {
    try {
      await invoke("save_password", { password })
      alert("Password saved successfully!")
      setPassword("") // Clear the password field after saving
      loadStoredHash() // Reload the stored hash
    } catch (error) {
      console.error("Failed to save password:", error)
      alert("Failed to save password. Please try again.")
    }
  }

  async function loadStoredHash() {
    try {
      const hash = await invoke("get_stored_hash")
      setStoredHash(hash as string)
    } catch (error) {
      console.error("Failed to load stored hash:", error)
      setStoredHash("No hash stored")
    }
  }

  return (
    <div className="container">
      <h1>Password Manager</h1>
      <div className="row">
        <input
          type="password"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          placeholder="Enter password..."
        />
        <button type="button" onClick={savePassword}>
          Save Password
        </button>
      </div>
      <div className="row">
        <h3>Stored Password Hash:</h3>
        <p>{storedHash}</p>
      </div>
    </div>
  )
}

export default App