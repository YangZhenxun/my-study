import mysql.connector

# 创建数据库连接
db = mysql.connector.connect(
    host="localhost",  # MySQL服务器地址
    user="root",   # 用户名
    password="Yzx20120413",  # 密码
    database="sys"  # 数据库名称
)

# 创建游标对象，用于执行SQL查询
cursor = db.cursor()
