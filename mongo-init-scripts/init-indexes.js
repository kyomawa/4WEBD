print("⚡ Script: Creating indexes for each database...");

// 1. AUTH DATABASE
db = db.getSiblingDB("auth-service");
// Get collection auth first to avoid conflict because db.auth is used by mongodb
db.getCollection("auth").createIndex(
  { user_id: 1 },
  { unique: true }
);
db.getCollection("auth").createIndex({ roles: 1 });

// 2. USERS DATABASE
db = db.getSiblingDB("users-service");
db.users.createIndex(
  { email: 1 },
  { unique: true }
);

// 3. NOTIFICATION DATABASE
db = db.getSiblingDB("notifications-service");
db.notifications.createIndex({ user_id: 1 });
db.notifications.createIndex({ status: 1 });

// 4. TICKETS DATABASE
db = db.getSiblingDB("tickets-service");
db.tickets.createIndex({ user_id: 1 });
db.tickets.createIndex({ event_id: 1 });
db.tickets.createIndex(
  { event_id: 1, seat_number: 1 },
  { unique: true }
);
db.tickets.createIndex({ user_id: 1, event_id: 1 });

// 5. EVENTS DATABASE
db = db.getSiblingDB("events-service");
db.events.createIndex({ date: 1 });
db.events.createIndex({ title: 1 }, { unique: true });
db.events.createIndex({ capacity: 1 });
db.events.createIndex({ remaining_seats: 1 });
db.events.createIndex({ creator_id: 1 });
db.events.createIndex({ created_at: 1 });

// 6. PAYMENTS DATABASE
db = db.getSiblingDB("payments-service");
db.payments.createIndex({ user_id: 1 });
db.payments.createIndex({ ticket_id: 1 });
db.payments.createIndex({ event_id: 1 });
db.payments.createIndex({ status: 1 });
db.payments.createIndex({ created_at: 1 });
