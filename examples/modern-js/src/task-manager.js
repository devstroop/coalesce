// React-style functional components using modern JavaScript patterns

import { createObservableState, createRateLimiter } from './api-client.js';

// Custom hooks pattern (functional programming)
export const useCounter = (initialValue = 0) => {
    const state = createObservableState({ count: initialValue });
    
    const increment = () => state.count += 1;
    const decrement = () => state.count -= 1;
    const reset = () => state.count = initialValue;
    const setCount = (value) => state.count = value;
    
    return {
        count: state.count,
        increment,
        decrement,
        reset,
        setCount,
        subscribe: state.subscribe
    };
};

// Advanced async patterns with concurrent operations
export const useAsyncData = (fetcher, dependencies = []) => {
    const state = createObservableState({
        data: null,
        loading: false,
        error: null,
        lastFetch: null
    });
    
    const controller = new AbortController();
    
    const fetchData = async (...args) => {
        if (state.loading) return; // Prevent concurrent fetches
        
        state.loading = true;
        state.error = null;
        
        try {
            const result = await fetcher(...args, { signal: controller.signal });
            state.data = result;
            state.lastFetch = Date.now();
        } catch (error) {
            if (error.name !== 'AbortError') {
                state.error = error.message;
            }
        } finally {
            state.loading = false;
        }
    };
    
    const refetch = () => fetchData(...dependencies);
    const cancel = () => controller.abort();
    
    // Auto-fetch when dependencies change
    let lastDeps = JSON.stringify(dependencies);
    const checkDependencies = () => {
        const currentDeps = JSON.stringify(dependencies);
        if (currentDeps !== lastDeps) {
            lastDeps = currentDeps;
            refetch();
        }
    };
    
    // Simulate effect hook behavior
    setTimeout(checkDependencies, 0);
    
    return {
        ...state,
        refetch,
        cancel,
        subscribe: state.subscribe
    };
};

// Complex component with multiple design patterns
export class TaskManager {
    #tasks = new Map();
    #subscribers = new Set();
    #rateLimiter = createRateLimiter(10, 60000); // 10 requests per minute
    
    constructor(options = {}) {
        this.options = {
            autoSave: true,
            persistKey: 'taskManager',
            maxTasks: 1000,
            ...options
        };
        
        this.#loadFromStorage();
    }
    
    // Task creation with validation and rate limiting
    async createTask({ 
        title, 
        description = '', 
        priority = 'medium', 
        dueDate = null, 
        tags = [], 
        assignee = null 
    }) {
        // Rate limiting
        const rateLimitResult = this.#rateLimiter('create');
        if (!rateLimitResult.allowed) {
            throw new Error(`Rate limit exceeded. Try again in ${rateLimitResult.resetTime - Date.now()}ms`);
        }
        
        // Validation
        if (!title?.trim()) {
            throw new Error('Task title is required');
        }
        
        if (this.#tasks.size >= this.options.maxTasks) {
            throw new Error(`Maximum task limit (${this.options.maxTasks}) reached`);
        }
        
        // Task creation
        const task = {
            id: crypto.randomUUID(),
            title: title.trim(),
            description: description.trim(),
            priority,
            status: 'pending',
            tags: [...new Set(tags)], // Remove duplicates
            assignee,
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            dueDate: dueDate ? new Date(dueDate).toISOString() : null
        };
        
        this.#tasks.set(task.id, task);
        this.#notifySubscribers('taskCreated', task);
        
        if (this.options.autoSave) {
            await this.#saveToStorage();
        }
        
        return task;
    }
    
    // Complex filtering and sorting with functional programming
    getTasks(filters = {}) {
        const {
            status,
            priority,
            assignee,
            tags,
            search,
            sortBy = 'createdAt',
            sortOrder = 'desc',
            limit,
            offset = 0
        } = filters;
        
        let tasks = Array.from(this.#tasks.values());
        
        // Apply filters using functional composition
        const applyFilters = pipe(
            // Status filter
            (tasks) => status ? tasks.filter(task => task.status === status) : tasks,
            
            // Priority filter
            (tasks) => priority ? tasks.filter(task => task.priority === priority) : tasks,
            
            // Assignee filter
            (tasks) => assignee ? tasks.filter(task => task.assignee === assignee) : tasks,
            
            // Tags filter (intersection)
            (tasks) => tags?.length 
                ? tasks.filter(task => tags.every(tag => task.tags.includes(tag)))
                : tasks,
            
            // Search filter
            (tasks) => search 
                ? tasks.filter(task => 
                    task.title.toLowerCase().includes(search.toLowerCase()) ||
                    task.description.toLowerCase().includes(search.toLowerCase())
                )
                : tasks,
            
            // Sorting
            (tasks) => tasks.sort((a, b) => {
                const aVal = a[sortBy];
                const bVal = b[sortBy];
                const comparison = aVal < bVal ? -1 : aVal > bVal ? 1 : 0;
                return sortOrder === 'asc' ? comparison : -comparison;
            }),
            
            // Pagination
            (tasks) => limit ? tasks.slice(offset, offset + limit) : tasks.slice(offset)
        );
        
        return applyFilters(tasks);
    }
    
    // Bulk operations with transaction-like behavior
    async updateTasks(taskIds, updates) {
        const originalTasks = new Map();
        const updatedTasks = [];
        
        try {
            // Begin "transaction"
            for (const id of taskIds) {
                const task = this.#tasks.get(id);
                if (!task) {
                    throw new Error(`Task ${id} not found`);
                }
                
                // Backup original
                originalTasks.set(id, { ...task });
                
                // Apply updates
                const updatedTask = {
                    ...task,
                    ...updates,
                    updatedAt: new Date().toISOString()
                };
                
                this.#tasks.set(id, updatedTask);
                updatedTasks.push(updatedTask);
            }
            
            // Commit "transaction"
            this.#notifySubscribers('tasksUpdated', updatedTasks);
            
            if (this.options.autoSave) {
                await this.#saveToStorage();
            }
            
            return updatedTasks;
            
        } catch (error) {
            // Rollback "transaction"
            for (const [id, originalTask] of originalTasks) {
                this.#tasks.set(id, originalTask);
            }
            throw error;
        }
    }
    
    // Complex aggregation queries
    getTaskStatistics() {
        const tasks = Array.from(this.#tasks.values());
        
        return {
            total: tasks.length,
            byStatus: this.#groupBy(tasks, 'status'),
            byPriority: this.#groupBy(tasks, 'priority'),
            byAssignee: this.#groupBy(tasks, 'assignee'),
            overdue: tasks.filter(task => 
                task.dueDate && new Date(task.dueDate) < new Date()
            ).length,
            dueSoon: tasks.filter(task => {
                if (!task.dueDate) return false;
                const dueDate = new Date(task.dueDate);
                const now = new Date();
                const threeDaysFromNow = new Date(now.getTime() + 3 * 24 * 60 * 60 * 1000);
                return dueDate > now && dueDate <= threeDaysFromNow;
            }).length,
            averageTasksPerDay: this.#calculateAverageTasksPerDay(tasks),
            completionRate: this.#calculateCompletionRate(tasks)
        };
    }
    
    // Observer pattern implementation
    subscribe(callback) {
        this.#subscribers.add(callback);
        return () => this.#subscribers.delete(callback);
    }
    
    #notifySubscribers(event, data) {
        this.#subscribers.forEach(callback => {
            try {
                callback({ event, data, timestamp: Date.now() });
            } catch (error) {
                console.error('Subscriber error:', error);
            }
        });
    }
    
    // Helper methods with advanced JavaScript features
    #groupBy(array, key) {
        return array.reduce((groups, item) => {
            const value = item[key] || 'unassigned';
            groups[value] = (groups[value] || 0) + 1;
            return groups;
        }, {});
    }
    
    #calculateAverageTasksPerDay(tasks) {
        if (tasks.length === 0) return 0;
        
        const dates = tasks.map(task => new Date(task.createdAt).toDateString());
        const uniqueDates = [...new Set(dates)];
        
        return Math.round((tasks.length / uniqueDates.length) * 100) / 100;
    }
    
    #calculateCompletionRate(tasks) {
        if (tasks.length === 0) return 0;
        
        const completed = tasks.filter(task => task.status === 'completed').length;
        return Math.round((completed / tasks.length) * 100 * 100) / 100; // Percentage
    }
    
    // Storage integration with error handling
    async #saveToStorage() {
        try {
            const data = JSON.stringify(Array.from(this.#tasks.entries()));
            localStorage.setItem(this.options.persistKey, data);
        } catch (error) {
            console.error('Failed to save tasks to storage:', error);
        }
    }
    
    #loadFromStorage() {
        try {
            const data = localStorage.getItem(this.options.persistKey);
            if (data) {
                const entries = JSON.parse(data);
                this.#tasks = new Map(entries);
            }
        } catch (error) {
            console.error('Failed to load tasks from storage:', error);
            this.#tasks = new Map();
        }
    }
}

// Factory pattern with dependency injection
export const createTaskManagerInstance = (dependencies = {}) => {
    const {
        storage = localStorage,
        rateLimiter = createRateLimiter(10, 60000),
        validator = (task) => task.title?.trim(),
        ...options
    } = dependencies;
    
    return new TaskManager({
        ...options,
        storage,
        rateLimiter,
        validator
    });
};

// Async iterator pattern for large data sets
export async function* processTasksBatch(tasks, batchSize = 10, processFn) {
    for (let i = 0; i < tasks.length; i += batchSize) {
        const batch = tasks.slice(i, i + batchSize);
        const results = await Promise.all(batch.map(processFn));
        yield {
            batchIndex: Math.floor(i / batchSize),
            processed: results,
            remaining: Math.max(0, tasks.length - i - batchSize)
        };
    }
}
