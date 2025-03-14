print("⚡ Script: Creating indexes for each database...");

// 1. AUTH DATABASE
db = db.getSiblingDB("auth-service");
db.credentials.createIndex(
  { user_id: 1 },
  { unique: true }
);
db.credentials.createIndex({ roles: 1 }); 

// 2. USERS DATABASE
db = db.getSiblingDB("users-service");
db.users.createIndex(
  { email: 1 },
  { unique: true }
);
db.users.createIndex({ phone: 1 }, { unique: true, sparse: true });

// 3. NOTIFICATION DATABASE
db = db.getSiblingDB("notification-service");
db.notifications.createIndex({ user_id: 1 });
db.notifications.createIndex({ status: 1 });

// 4. TICKETS DATABASE
db = db.getSiblingDB("tickets-service");
db.tickets.createIndex({ user_id: 1 });
db.tickets.createIndex({ event_id: 1 });
db.tickets.createIndex({ user_id: 1, event_id: 1 });

// 5. EVENTS DATABASE
db = db.getSiblingDB("events-service");
db.events.createIndex({ date: 1 });
db.events.createIndex({ title: 1 });
db.events.createIndex({ capacity: 1 });
db.events.createIndex({ remaining_seats: 1 });
db.events.createIndex({ creator_id: 1 });