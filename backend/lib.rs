use candid::{CandidType, Deserialize};
use ic_cdk::query;

#[derive(CandidType, Deserialize)]
struct LegalQuery {
    question: String,
}

#[derive(CandidType, Deserialize)]
struct LegalResponse {
    answer: String,
}

#[query]
fn get_legal_advice(query: LegalQuery) -> LegalResponse {
    // Simple implementation with hardcoded responses
    let question = query.question.to_lowercase();
    
    let answer = if question.contains("contract") {
        "Contracts require offer, acceptance, consideration, and legal intent to be valid. Always read before signing and consider consulting a lawyer for complex agreements."
    } else if question.contains("copyright") {
        "Copyright protects original works of authorship. Registration provides additional benefits but isn't required for protection in most countries."
    } else if question.contains("liability") {
        "Liability can be limited through proper disclaimers and terms of service, but cannot be completely eliminated in many jurisdictions."
    } else {
        "I'm a simple legal advisor bot. Please ask about contracts, copyright, or liability. Note that this is not professional legal advice."
    };
    
    LegalResponse {
        answer: answer.to_string(),
    }
}

// Export Candid interface
ic_cdk::export_candid!();