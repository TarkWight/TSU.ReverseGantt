const jwt = {
    decode(token) {
        try {
            const parts = token.split('.');
            if (parts.length !== 3) {
                return null;
            }

            const payload = parts[1];
            const decoded = atob(payload.replace(/-/g, '+').replace(/_/g, '/'));
            return JSON.parse(decoded);
        } catch (error) {
            console.error('JWT decode error:', error);
            return null;
        }
    },

    getClaims(token) {
        const decoded = this.decode(token);
        if (!decoded) {
            return null;
        }

        return {
            userId: decoded.sub,
            globalRole: decoded.role,
            name: decoded.name,
            exp: decoded.exp,
        };
    },

    isExpired(token) {
        const claims = this.getClaims(token);
        if (!claims || !claims.exp) {
            return true;
        }

        const now = Math.floor(Date.now() / 1000);
        return claims.exp < now;
    },
};

