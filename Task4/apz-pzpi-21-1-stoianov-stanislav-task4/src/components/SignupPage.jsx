import React, { useState } from "react";
import { useNavigate } from "react-router-dom";
import { useLocale } from "../locale";

function SignupPage() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();
  const locale = useLocale();

  const handleSignup = () => {
    const formData = new URLSearchParams();
    formData.append("email", email);
    formData.append("password", password);

    fetch("http://localhost:8080/signup", {
      method: "POST",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: formData,
      credentials: "include",
    })
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to signup");
        }
        navigate("/login");
      })
      .catch((error) => console.log(error.message));
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">{`${locale.signup}`}</h1>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.email}`}</label>
        <input
          type="email"
          className="w-full p-2 border border-gray-300"
          value={email}
          onChange={(e) => setEmail(e.target.value)}
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.password}`}</label>
        <input
          type="password"
          className="w-full p-2 border border-gray-300"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
      </div>
      <button
        className="bg-blue-500 text-white px-4 py-2 rounded"
        onClick={handleSignup}
      >
        {`${locale.signup}`}
      </button>
    </div>
  );
}

export default SignupPage;
