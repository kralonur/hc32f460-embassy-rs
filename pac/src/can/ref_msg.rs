#[doc = "Register `REF_MSG` reader"]
pub type R = crate::R<RefMsgSpec>;
#[doc = "Register `REF_MSG` writer"]
pub type W = crate::W<RefMsgSpec>;
#[doc = "Field `REF_ID` reader - desc REF_ID"]
pub type RefIdR = crate::FieldReader<u32>;
#[doc = "Field `REF_ID` writer - desc REF_ID"]
pub type RefIdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `REF_IDE` reader - desc REF_IDE"]
pub type RefIdeR = crate::BitReader;
#[doc = "Field `REF_IDE` writer - desc REF_IDE"]
pub type RefIdeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - desc REF_ID"]
    #[inline(always)]
    pub fn ref_id(&self) -> RefIdR {
        RefIdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 31 - desc REF_IDE"]
    #[inline(always)]
    pub fn ref_ide(&self) -> RefIdeR {
        RefIdeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - desc REF_ID"]
    #[inline(always)]
    pub fn ref_id(&mut self) -> RefIdW<'_, RefMsgSpec> {
        RefIdW::new(self, 0)
    }
    #[doc = "Bit 31 - desc REF_IDE"]
    #[inline(always)]
    pub fn ref_ide(&mut self) -> RefIdeW<'_, RefMsgSpec> {
        RefIdeW::new(self, 31)
    }
}
#[doc = "desc REF_MSG\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_msg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_msg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefMsgSpec;
impl crate::RegisterSpec for RefMsgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_msg::R`](R) reader structure"]
impl crate::Readable for RefMsgSpec {}
#[doc = "`write(|w| ..)` method takes [`ref_msg::W`](W) writer structure"]
impl crate::Writable for RefMsgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_MSG to value 0"]
impl crate::Resettable for RefMsgSpec {}
