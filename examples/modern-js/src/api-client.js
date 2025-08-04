// Modern JavaScript patterns for testing the Coalesce parser

// ES6+ Class with async methods and decorators
class UserService {
    #apiKey = null;
    
    constructor(apiEndpoint) {
        this.endpoint = apiEndpoint;
        this.cache = new Map();
    }
    
    // Arrow function as class method
    validateUser = async (userId) => {
        if (!userId) {
            throw new Error('User ID is required');
        }
        
        // Template literals with expressions
        const cacheKey = `user:${userId}:${Date.now()}`;
        
        // Destructuring with default values
        const { data: userData = null, error = null } = await this.fetchUser(userId);
        
        if (error) {
            console.error(`Failed to fetch user ${userId}:`, error);
            return { valid: false, error };
        }
        
        return { valid: true, user: userData };
    };
    
    // Generator function with yield
    * getUserBatch(userIds) {
        for (const id of userIds) {
            const user = this.cache.get(id);
            if (user) {
                yield user;
            } else {
                // Promise-based async operation
                const fetchedUser = yield this.fetchUser(id);
                this.cache.set(id, fetchedUser);
                yield fetchedUser;
            }
        }
    }
    
    // Private method (ES2022)
    #buildAuthHeaders() {
        return {
            'Authorization': `Bearer ${this.#apiKey}`,
            'Content-Type': 'application/json',
            'X-API-Version': '2024'
        };
    }
    
    // Static method with complex parameter patterns
    static transformUserData({ 
        id, 
        name, 
        email, 
        preferences = {}, 
        ...metadata 
    }) {
        // Object spread and computed properties
        return {
            userId: id,
            displayName: name?.trim() || 'Anonymous',
            contactInfo: {
                email: email?.toLowerCase(),
                verified: metadata.emailVerified ?? false
            },
            settings: {
                theme: preferences.theme || 'light',
                notifications: preferences.notifications ?? true,
                ...preferences.custom
            }
        };
    }
    
    // Async/await with error handling
    async fetchUser(userId) {
        try {
            // Fetch with AbortController for cancellation
            const controller = new AbortController();
            const timeoutId = setTimeout(() => controller.abort(), 5000);
            
            const response = await fetch(`${this.endpoint}/users/${userId}`, {
                method: 'GET',
                headers: this.#buildAuthHeaders(),
                signal: controller.signal
            });
            
            clearTimeout(timeoutId);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            const data = await response.json();
            return { data, error: null };
            
        } catch (error) {
            console.error('Fetch error:', error);
            return { data: null, error: error.message };
        }
    }
}

// Module exports and imports pattern
export { UserService };
export default UserService;

// Higher-order function with closures
export const createRateLimiter = (maxRequests, timeWindow) => {
    const requests = new Map();
    
    return (identifier) => {
        const now = Date.now();
        const windowStart = now - timeWindow;
        
        // Clean old entries
        for (const [key, timestamps] of requests) {
            const filtered = timestamps.filter(time => time > windowStart);
            if (filtered.length === 0) {
                requests.delete(key);
            } else {
                requests.set(key, filtered);
            }
        }
        
        // Check current request count
        const currentRequests = requests.get(identifier) || [];
        if (currentRequests.length >= maxRequests) {
            return { allowed: false, resetTime: Math.min(...currentRequests) + timeWindow };
        }
        
        // Add current request
        currentRequests.push(now);
        requests.set(identifier, currentRequests);
        
        return { allowed: true, remaining: maxRequests - currentRequests.length };
    };
};

// Functional programming patterns
const pipe = (...functions) => (value) => 
    functions.reduce((acc, fn) => fn(acc), value);

const curry = (fn) => (...args) => 
    args.length >= fn.length 
        ? fn(...args) 
        : (...nextArgs) => curry(fn)(...args, ...nextArgs);

// Complex object patterns with Proxy
export const createObservableState = (initialState = {}) => {
    const observers = new Set();
    
    const notify = (property, oldValue, newValue) => {
        observers.forEach(observer => {
            observer({ property, oldValue, newValue, state: target });
        });
    };
    
    const target = { ...initialState };
    
    return new Proxy(target, {
        set(obj, property, value) {
            const oldValue = obj[property];
            obj[property] = value;
            notify(property, oldValue, value);
            return true;
        },
        
        get(obj, property) {
            if (property === 'subscribe') {
                return (observer) => {
                    observers.add(observer);
                    return () => observers.delete(observer);
                };
            }
            return obj[property];
        }
    });
};

// Web API integrations and modern patterns
export class ApiClient {
    #baseUrl;
    #interceptors = {
        request: [],
        response: []
    };
    
    constructor(baseUrl, options = {}) {
        this.#baseUrl = baseUrl.replace(/\/$/, '');
        this.defaultOptions = {
            timeout: 10000,
            retries: 3,
            ...options
        };
    }
    
    // Method chaining pattern
    addRequestInterceptor(interceptor) {
        this.#interceptors.request.push(interceptor);
        return this;
    }
    
    addResponseInterceptor(interceptor) {
        this.#interceptors.response.push(interceptor);
        return this;
    }
    
    // Generic HTTP method with retry logic
    async request(endpoint, options = {}) {
        const url = `${this.#baseUrl}${endpoint}`;
        let config = { ...this.defaultOptions, ...options };
        
        // Apply request interceptors
        for (const interceptor of this.#interceptors.request) {
            config = await interceptor(config);
        }
        
        let lastError;
        
        for (let attempt = 0; attempt < config.retries; attempt++) {
            try {
                const response = await this.#performRequest(url, config);
                
                // Apply response interceptors
                let processedResponse = response;
                for (const interceptor of this.#interceptors.response) {
                    processedResponse = await interceptor(processedResponse);
                }
                
                return processedResponse;
                
            } catch (error) {
                lastError = error;
                
                if (attempt < config.retries - 1) {
                    // Exponential backoff
                    const delay = Math.pow(2, attempt) * 1000;
                    await new Promise(resolve => setTimeout(resolve, delay));
                }
            }
        }
        
        throw lastError;
    }
    
    async #performRequest(url, config) {
        const controller = new AbortController();
        const timeoutId = setTimeout(() => controller.abort(), config.timeout);
        
        try {
            const response = await fetch(url, {
                ...config,
                signal: controller.signal
            });
            
            clearTimeout(timeoutId);
            
            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }
            
            return response;
        } catch (error) {
            clearTimeout(timeoutId);
            throw error;
        }
    }
    
    // Convenience methods using method delegation
    get = (endpoint, options = {}) => 
        this.request(endpoint, { ...options, method: 'GET' });
    
    post = (endpoint, data, options = {}) => 
        this.request(endpoint, { 
            ...options, 
            method: 'POST', 
            body: JSON.stringify(data),
            headers: { 'Content-Type': 'application/json', ...options.headers }
        });
    
    put = (endpoint, data, options = {}) => 
        this.request(endpoint, { 
            ...options, 
            method: 'PUT', 
            body: JSON.stringify(data),
            headers: { 'Content-Type': 'application/json', ...options.headers }
        });
    
    delete = (endpoint, options = {}) => 
        this.request(endpoint, { ...options, method: 'DELETE' });
}
