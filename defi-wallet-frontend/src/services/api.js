import axios from 'axios';

const API_URL = 'http://localhost:8080'; // Adjust this based on your backend URL

export const createWallet = async () => {
    const response = await axios.get(`${API_URL}/create_wallet`);
    return response.data;
};

export const getBalance = async (address) => {
    const response = await axios.get(`${API_URL}/balance/${address}`);
    return response.data;
};

// Add more API methods as needed
