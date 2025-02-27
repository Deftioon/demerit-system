import React, { createContext, useContext, useState, useEffect } from "react";

interface User {
  id: string;
  email: string;
  username?: string;
  firstName?: string;
  lastName?: string;
  userType: string; // // 'teacher' | 'student' | 'parent'
}

interface UserContextType {
  user: User | null;
  setUser: (user: User | null) => void;
  logout: () => void;
}

const UserContext = createContext<UserContextType | undefined>(undefined);

export const UserProvider: React.FC<{ children: React.ReactNode }> = ({
  children,
}) => {
  const [user, setUserState] = useState<User | null>(() => {
    // Load user from localStorage on initial render
    const savedUser = localStorage.getItem("user");
    return savedUser ? JSON.parse(savedUser) : null;
  });

  // Update localStorage when user changes
  useEffect(() => {
    if (user) {
      localStorage.setItem("user", JSON.stringify(user));
    } else {
      localStorage.removeItem("user");
    }
  }, [user]);

  const setUser = (newUser: User | null) => {
    setUserState(newUser);
  };

  const logout = () => {
    localStorage.removeItem("user");
    setUserState(null);
  };

  return (
    <UserContext.Provider value={{ user, setUser, logout }}>
      {children}
    </UserContext.Provider>
  );
};

export const useUser = () => {
  const context = useContext(UserContext);
  if (context === undefined) {
    throw new Error("useUser must be used within a UserProvider");
  }
  return context;
};
