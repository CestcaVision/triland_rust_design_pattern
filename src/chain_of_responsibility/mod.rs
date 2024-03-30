pub trait MedicalStep {
    fn set_next(&mut self, next: Box<dyn MedicalStep>);
    fn handle(&self, request: &Patient);
}

pub struct Patient {
    pub name: String,
    // 这里可以增加更多病人相关的属性，例如病情描述，是否已治疗等
}

// 挂号步骤
pub struct Registration {
    next: Option<Box<dyn MedicalStep>>,
}

impl MedicalStep for Registration {
    fn set_next(&mut self, next: Box<dyn MedicalStep>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &Patient) {
        println!("{} is registering.", request.name);
        // 假设每个步骤都有可能根据条件判断是否需要移交到下一个步骤
        if let Some(ref next_step) = self.next {
            next_step.handle(request);
        }
    }
}

// 缴费步骤
pub struct Payment {
    next: Option<Box<dyn MedicalStep>>,
}

impl MedicalStep for Payment {
    fn set_next(&mut self, next: Box<dyn MedicalStep>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &Patient) {
        println!("{} is making a payment.", request.name);
        if let Some(ref next_step) = self.next {
            next_step.handle(request);
        }
    }
}

// 看医生步骤
pub struct Consultation {
    next: Option<Box<dyn MedicalStep>>,
}

impl MedicalStep for Consultation {
    fn set_next(&mut self, next: Box<dyn MedicalStep>) {
        self.next = Some(next);
    }

    fn handle(&self, request: &Patient) {
        println!("{} is consulting with a doctor.", request.name);
        if let Some(ref next_step) = self.next {
            next_step.handle(request);
        }
    }
}

// 取药步骤
pub struct Pharmacy {
    next: Option<Box<dyn MedicalStep>>,
}

impl MedicalStep for Pharmacy {
    fn set_next(&mut self, _next: Box<dyn MedicalStep>) {
        // 没有下一步
    }

    fn handle(&self, request: &Patient) {
        println!("{} is getting medicine.", request.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_of_responsibility_pattern_functionality() {
        let mut registration = Box::new(Registration { next: None });
        let mut payment = Box::new(Payment { next: None });
        let mut consultation = Box::new(Consultation { next: None });
        let pharmacy = Box::new(Pharmacy { next: None });

        // 设置责任链
        consultation.set_next(pharmacy);
        payment.set_next(consultation);
        registration.set_next(payment);

        let patient = Patient {
            name: String::from("John Doe"),
        };

        // 处理请求
        registration.handle(&patient);
    }
}
