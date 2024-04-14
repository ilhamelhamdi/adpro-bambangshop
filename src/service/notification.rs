use crate::model::notification::Notification;
use crate::model::product::Product;
use crate::model::subscriber::Subscriber;
use crate::repository::subscriber::SubscriberRepository;
use bambangshop::{compose_error_response, Result};
use std::thread;

pub struct NotificationService;

impl NotificationService {}
