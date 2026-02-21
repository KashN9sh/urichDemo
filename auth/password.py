"""Password hashing via bcrypt. Long passwords are pre-hashed with SHA256 to fit bcrypt's 72-byte limit."""
import hashlib

import bcrypt

_MAX_BCRYPT_BYTES = 72


def _to_bcrypt_input(password: str) -> bytes:
    """Normalize input for bcrypt: if 72+ bytes in UTF-8, use SHA256 hex so bcrypt never sees >71 bytes."""
    raw = password.encode("utf-8")
    if len(raw) < _MAX_BCRYPT_BYTES:
        return raw
    return hashlib.sha256(raw).hexdigest().encode("ascii")


def hash_password(password: str) -> str:
    return bcrypt.hashpw(_to_bcrypt_input(password), bcrypt.gensalt()).decode("utf-8")


def verify_password(plain: str, hashed: str) -> bool:
    return bcrypt.checkpw(_to_bcrypt_input(plain), hashed.encode("utf-8"))
