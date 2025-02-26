use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref TRANSLATIONS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Sunday", "일요일");
        m.insert("Monday", "월요일");
        m.insert("Tuesday", "화요일");
        m.insert("Wednesday", "수요일");
        m.insert("Thursday", "목요일");
        m.insert("Friday", "금요일");
        m.insert("Saturday", "토요일");
        m.insert("January", "1월");
        m.insert("February", "2월");
        m.insert("March", "3월");
        m.insert("April", "4월");
        m.insert("May", "5월");
        m.insert("June", "6월");
        m.insert("July", "7월");
        m.insert("August", "8월");
        m.insert("September", "9월");
        m.insert("October", "10월");
        m.insert("November", "11월");
        m.insert("December", "12월");
        m.insert("Settings", "설정");
        m.insert("seconds ago", "초 전");
        m.insert("_%n minute ago_::_%n minutes ago_", "%n분 전 ");
        m.insert("_%n hour ago_::_%n hours ago_", "%n시간 전 ");
        m.insert("today", "오늘");
        m.insert("yesterday", "어제");
        m.insert("_%n day ago_::_%n days ago_", "%n일 전 ");
        m.insert("last month", "지난 달");
        m.insert("_%n month ago_::_%n months ago_", "%n달 전 ");
        m.insert("months ago", "개월 전");
        m.insert("last year", "작년");
        m.insert("years ago", "년 전");
        m.insert("Choose", "선택");
        m.insert("Yes", "예");
        m.insert("No", "아니요");
        m.insert("Ok", "승락");
        m.insert("_{count} file conflict_::_{count} file conflicts_", "");
        m.insert("Cancel", "취소");
        m.insert("Shared", "공유됨");
        m.insert("Share", "공유");
        m.insert("Error", "오류");
        m.insert("Error while sharing", "공유하는 중 오류 발생");
        m.insert("Error while unsharing", "공유 해제하는 중 오류 발생");
        m.insert("Error while changing permissions", "권한 변경하는 중 오류 발생");
        m.insert("Shared with you and the group {group} by {owner}", "{owner} 님이 여러분 및 그룹 {group}와(과) 공유 중");
        m.insert("Shared with you by {owner}", "{owner} 님이 공유 중");
        m.insert("Password protect", "암호 보호");
        m.insert("Password", "암호");
        m.insert("Allow Public Upload", "퍼블릭 업로드 허용");
        m.insert("Email link to person", "이메일 주소");
        m.insert("Send", "전송");
        m.insert("Set expiration date", "만료 날짜 설정");
        m.insert("Expiration date", "만료 날짜");
        m.insert("Share via email:", "이메일로 공유:");
        m.insert("No people found", "발견된 사람 없음");
        m.insert("group", "그룹");
        m.insert("Resharing is not allowed", "다시 공유할 수 없습니다");
        m.insert("Shared in {item} with {user}", "{user} 님과 {item}에서 공유 중");
        m.insert("Unshare", "공유 해제");
        m.insert("can edit", "편집 가능");
        m.insert("access control", "접근 제어");
        m.insert("create", "생성");
        m.insert("update", "업데이트");
        m.insert("delete", "삭제");
        m.insert("share", "공유");
        m.insert("Password protected", "암호로 보호됨");
        m.insert("Error unsetting expiration date", "만료 날짜 해제 오류");
        m.insert("Error setting expiration date", "만료 날짜 설정 오류");
        m.insert("Sending ...", "전송 중...");
        m.insert("Email sent", "이메일 발송됨");
        m.insert("Warning", "경고");
        m.insert("The object type is not specified.", "객체 유형이 지정되지 않았습니다.");
        m.insert("Delete", "삭제");
        m.insert("Add", "추가");
        m.insert("The update was unsuccessful. Please report this issue to the <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud community</a>.", "업데이트가 실패하였습니다. 이 문제를 <a href=\"https://github.com/owncloud/core/issues\" target=\"_blank\">ownCloud 커뮤니티</a>에 보고해 주십시오.");
        m.insert("The update was successful. Redirecting you to ownCloud now.", "업데이트가 성공하였습니다. ownCloud로 돌아갑니다.");
        m.insert("Use the following link to reset your password: {link}", "다음 링크를 사용하여 암호를 재설정할 수 있습니다: {link}");
        m.insert("Request failed!<br>Did you make sure your email/username was right?", "요청이 실패했습니다!<br>email 주소와 사용자 명을 정확하게 넣으셨나요?");
        m.insert("You will receive a link to reset your password via Email.", "이메일로 암호 재설정 링크를 보냈습니다.");
        m.insert("Username", "사용자 이름");
        m.insert("Yes, I really want to reset my password now", "네, 전 제 비밀번호를 리셋하길 원합니다");
        m.insert("Your password was reset", "암호가 재설정되었습니다");
        m.insert("To login page", "로그인 화면으로");
        m.insert("New password", "새 암호");
        m.insert("Reset password", "암호 재설정");
        m.insert("Personal", "개인");
        m.insert("Users", "사용자");
        m.insert("Apps", "앱");
        m.insert("Admin", "관리자");
        m.insert("Help", "도움말");
        m.insert("Access forbidden", "접근 금지됨");
        m.insert("Cloud not found", "클라우드를 찾을 수 없습니다");
        m.insert("Security Warning", "보안 경고");
        m.insert("Your PHP version is vulnerable to the NULL Byte attack (CVE-2006-7243)", "사용 중인 PHP 버전이 NULL 바이트 공격에 취약합니다 (CVE-2006-7243)");
        m.insert("No secure random number generator is available, please enable the PHP OpenSSL extension.", "안전한 난수 생성기를 사용할 수 없습니다. PHP의 OpenSSL 확장을 활성화해 주십시오.");
        m.insert("Without a secure random number generator an attacker may be able to predict password reset tokens and take over your account.", "안전한 난수 생성기를 사용하지 않으면 공격자가 암호 초기화 토큰을 추측하여 계정을 탈취할 수 있습니다.");
        m.insert("Your data directory and files are probably accessible from the internet because the .htaccess file does not work.", ".htaccess 파일이 처리되지 않아서 데이터 디렉터리와 파일을 인터넷에서 접근할 수 없을 수도 있습니다.");
        m.insert("Create an <strong>admin account</strong>", "<strong>관리자 계정</strong> 만들기");
        m.insert("Advanced", "고급");
        m.insert("Data folder", "데이터 폴더");
        m.insert("Configure the database", "데이터베이스 설정");
        m.insert("will be used", "사용될 예정");
        m.insert("Database user", "데이터베이스 사용자");
        m.insert("Database password", "데이터베이스 암호");
        m.insert("Database name", "데이터베이스 이름");
        m.insert("Database tablespace", "데이터베이스 테이블 공간");
        m.insert("Database host", "데이터베이스 호스트");
        m.insert("Finish setup", "설치 완료");
        m.insert("Log out", "로그아웃");
        m.insert("Automatic logon rejected!", "자동 로그인이 거부되었습니다!");
        m.insert("If you did not change your password recently, your account may be compromised!", "최근에 암호를 변경하지 않았다면 계정이 탈취되었을 수도 있습니다!");
        m.insert("Please change your password to secure your account again.", "계정의 안전을 위하여 암호를 변경하십시오.");
        m.insert("Lost your password?", "암호를 잊으셨습니까?");
        m.insert("remember", "기억하기");
        m.insert("Log in", "로그인");
        m.insert("Alternative Logins", "대체 ");
        m.insert("Updating ownCloud to version %s, this may take a while.", "ownCloud를 버전 %s(으)로 업데이트합니다. 잠시 기다려 주십시오.");
        m
    };

    pub static ref PLURAL_FORMS: &'static str = "nplurals=1; plural=0;";
}