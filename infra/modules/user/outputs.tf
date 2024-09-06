output "login_profile" {
  value = {
    username = aws_iam_user.user.name
    password = try(aws_iam_user_login_profile.user[0].encrypted_password, "")
  }
}

output "access_key" {
  value = {
    id     = try(aws_iam_access_key.user[0].id, "")
    secret = try(aws_iam_access_key.user[0].secret, "")
  }
}
