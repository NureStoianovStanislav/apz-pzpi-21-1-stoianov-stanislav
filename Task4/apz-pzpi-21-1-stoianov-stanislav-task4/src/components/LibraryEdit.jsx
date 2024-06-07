import React, { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { useLocale } from "../locale";

function LibraryDetails() {
  const { id } = useParams();
  const navigate = useNavigate();
  const [library, setLibrary] = useState({
    name: "",
    address: "",
    dailyRate: "",
    overdueRate: "",
    currency: "",
    ownerId: "",
  });
  const [users, setUsers] = useState([]);
  const locale = useLocale();

  useEffect(() => {
    fetch(`http://localhost:8080/libraries/${id}`)
      .then((response) => {
        if (response.status === 401 || response.status === 403) {
          navigate("/login");
        }
        if (response.ok) {
          return response.json();
        }
        throw new Error("Failed to fetch library details");
      })
      .then((data) => setLibrary(data))
      .catch((error) => console.log(error.message));

    fetch("http://localhost:8080/auth/users", { credentials: "include" })
      .then((response) => {
        if (!response.ok) {
          throw new Error("Failed to fetch users");
        }
        return response.json();
      })
      .then((data) => setUsers(data))
      .catch((error) => console.log(error.message));
  }, [id, navigate]);

  const handleChange = (e) => {
    const { name, value } = e.target;
    setLibrary((prevState) => ({
      ...prevState,
      [name]: value,
    }));
  };

  const handleUpdate = () => {
    const formData = new URLSearchParams();
    Object.keys(library).forEach((key) => {
      if (key === "rating") return;
      formData.append(key, library[key]);
    });

    fetch(`http://localhost:8080/libraries/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/x-www-form-urlencoded",
      },
      body: formData,
      credentials: "include",
    })
      .then((response) => {
        if (response.status === 401 || response.status === 403) {
          navigate("/login");
        }
        if (!response.ok) {
          throw new Error("Failed to update library");
        }
        console.log("Library updated successfully");
      })
      .catch((error) => console.log(error.message));
  };

  const handleDelete = () => {
    if (!window.confirm(locale.deleteConfirmation)) return;
    fetch(`http://localhost:8080/libraries/${id}`, {
      method: "DELETE",
      credentials: "include",
    })
      .then((response) => {
        if (response.status === 401 || response.status === 403) {
          navigate("/login");
        }
        if (!response.ok) {
          throw new Error("Failed to delete library");
        }
        console.log("Library deleted successfully");
        navigate("/libraries");
      })
      .catch((error) => console.log(error.message));
  };

  return (
    <div className="p-4">
      <h1 className="text-2xl font-bold mb-4">{`${locale.libraryDetailsTitle}`}</h1>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.name}`}</label>
        <input
          type="text"
          name="name"
          className="w-full p-2 border border-gray-300"
          value={library.name}
          onChange={handleChange}
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.address}`}</label>
        <input
          type="text"
          name="address"
          className="w-full p-2 border border-gray-300"
          value={library.address}
          onChange={handleChange}
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.dailyRate}`}</label>
        <input
          type="text"
          name="dailyRate"
          className="w-full p-2 border border-gray-300"
          value={library.dailyRate}
          onChange={handleChange}
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.overdueRate}`}</label>
        <input
          type="text"
          name="overdueRate"
          className="w-full p-2 border border-gray-300"
          value={library.overdueRate}
          onChange={handleChange}
        />
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.currency}`}</label>
        <select
          name="currency"
          value={library.currency}
          onChange={handleChange}
          className="w-full p-2 border border-gray-300"
        >
          <option value="">{`${locale.selectCurrency}`}</option>
          <option value="UAH">UAH</option>
          <option value="USD">USD</option>
          <option value="EUR">EUR</option>
        </select>
      </div>
      <div className="mb-4">
        <label className="block mb-2">{`${locale.owner}`}</label>
        <select
          name="ownerId"
          value={library.ownerId}
          onChange={handleChange}
          className="w-full p-2 border border-gray-300"
        >
          <option value="">{`${locale.selectOwner}`}</option>
          {users.map((user) => (
            <option key={user.id} value={user.id}>
              {user.name} - {user.email}
            </option>
          ))}
        </select>
      </div>
      <button
        className="bg-blue-500 text-white px-4 py-2 rounded"
        onClick={handleUpdate}
      >
        {`${locale.update}`}
      </button>
      <button
        className="bg-red-500 text-white px-4 py-2 rounded ml-2"
        onClick={handleDelete}
      >
        {`${locale.delete}`}
      </button>
    </div>
  );
}

export default LibraryDetails;
